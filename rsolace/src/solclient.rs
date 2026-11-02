use super::solcache::CacheSessionProps;
use super::solevent::SolEvent;
use super::solmsg::{SolMsg, SolMsgError};
// Re-export for backward compatibility
pub use super::solprops::SessionProps;
use super::types::{
    ErrorInfo, SolClientCacheRequestFlags, SolClientLogLevel, SolClientReturnCode,
    SolClientSessionEvent, SolClientSubscribeFlags,
};
use super::utils::ConvertToCString;
use dashmap::DashMap;
use enum_primitive::FromPrimitive;
use snafu::prelude::{ensure, Snafu};
use snafu::ResultExt;
// use std::borrow::Cow;
use std::ffi::{c_void, CString};
use std::option::Option;
use std::os::raw::c_char;
use std::pin::Pin;
use std::ptr::{null, null_mut};
use std::time::Duration;
// TODO fn pointer to struct
#[cfg(feature = "channel")]
use kanal::{bounded, unbounded, Receiver, Sender};
// Async kanal imports for future async support
#[cfg(all(feature = "channel", feature = "tokio"))]
use kanal::{bounded_async, AsyncReceiver, AsyncSender};
#[cfg(feature = "channel")]
use std::collections::VecDeque;
#[cfg(feature = "channel")]
use std::sync::atomic::AtomicU64;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
#[cfg(feature = "channel")]
use std::sync::Mutex;
#[cfg(feature = "channel")]
use std::time::Instant;

/// Maximum time `modify_client_info` will wait for the asynchronous
/// `SOLCLIENT_SESSION_EVENT_MODIFYPROP_OK` / `_FAIL` confirmation.
///
/// Matches Solace's documented default of
/// `SOLCLIENT_SESSION_PROP_DEFAULT_MODIFYPROP_TIMEOUT_MS = 10000`.
const MODIFY_CLIENT_INFO_TIMEOUT_MS: u64 = 10_000;

/// Most messages one [`SolClient::send_multiple_msg_written`] call accepts
/// (`SOLCLIENT_SESSION_SEND_MULTIPLE_LIMIT`).
pub const SEND_MULTIPLE_LIMIT: usize = rsolace_sys::SOLCLIENT_SESSION_SEND_MULTIPLE_LIMIT as usize;

/// How long a request that the caller gave up on (timed out or cancelled)
/// is remembered, so a reply that still arrives can be logged with its topic.
#[cfg(feature = "channel")]
const ABANDONED_REQUEST_TTL: Duration = Duration::from_secs(600);

/// Most abandoned requests remembered at once; the oldest is forgotten first.
#[cfg(feature = "channel")]
const ABANDONED_REQUEST_CAP: usize = 1024;

#[cfg(feature = "channel")]
struct AbandonedRequest {
    corr_id: String,
    topic: String,
    at: Instant,
}

/// Small bounded record of requests whose caller stopped waiting, keyed by
/// correlation id. Bounded by count and by age.
#[cfg(feature = "channel")]
#[derive(Default)]
struct AbandonedRequests {
    entries: Mutex<VecDeque<AbandonedRequest>>,
}

/// Abandoned-request queue, borrowed under its lock.
///
/// Every move of a request from a pending map into this queue, and every
/// lookup by the message callback after a pending miss, happens while this
/// lock is held. Timestamps are taken after the lock is acquired, so the
/// queue stays in time order and pruning from the front is enough.
#[cfg(feature = "channel")]
struct AbandonedQueue<'a> {
    entries: std::sync::MutexGuard<'a, VecDeque<AbandonedRequest>>,
    now: Instant,
}

#[cfg(feature = "channel")]
impl AbandonedQueue<'_> {
    fn prune(&mut self) {
        while let Some(front) = self.entries.front() {
            if self.now.saturating_duration_since(front.at) > ABANDONED_REQUEST_TTL {
                self.entries.pop_front();
            } else {
                break;
            }
        }
    }

    fn record(&mut self, corr_id: String, topic: String) {
        self.prune();
        while self.entries.len() >= ABANDONED_REQUEST_CAP {
            self.entries.pop_front();
        }
        self.entries.push_back(AbandonedRequest {
            corr_id,
            topic,
            at: self.now,
        });
    }

    /// Remove and return the topic of an abandoned request, if remembered.
    fn take(&mut self, corr_id: &str) -> Option<String> {
        self.prune();
        let idx = self.entries.iter().rposition(|e| e.corr_id == corr_id)?;
        self.entries.remove(idx).map(|e| e.topic)
    }
}

#[cfg(feature = "channel")]
impl AbandonedRequests {
    /// Lock the queue, then read the clock.
    fn lock(&self) -> AbandonedQueue<'_> {
        let entries = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        AbandonedQueue {
            entries,
            now: Instant::now(),
        }
    }

    fn clear(&self) {
        self.lock().entries.clear();
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.lock().entries.len()
    }
}

/// An in-flight request waiting for its reply, keyed by correlation id in
/// `request_reply_map`. `id` is unique per request so removal can check it
/// is removing its own entry and not a later request reusing the corr id.
#[cfg(feature = "channel")]
struct PendingSyncRequest {
    id: u64,
    topic: String,
    sender: Sender<SolMsg>,
}

/// Async counterpart of [`PendingSyncRequest`], in `async_request_reply_map`.
#[cfg(all(feature = "channel", feature = "tokio"))]
struct PendingAsyncRequest {
    id: u64,
    topic: String,
    sender: AsyncSender<SolMsg>,
}

/// What happened to an incoming reply.
#[cfg(feature = "channel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplyOutcome {
    /// The reply was handed to the waiting receiver.
    Delivered,
    /// The requester already gave up (timed out or cancelled); the reply is
    /// discarded and logged at WARN.
    LateReply,
    /// The reply channel was unexpectedly full; the reply is discarded.
    ChannelFull,
    /// No pending or recently abandoned request has this corr id.
    Unmatched,
}

#[cfg(feature = "channel")]
fn log_late_reply(corr_id: &str, topic: &str) {
    tracing::warn!(
        "late reply discarded, requester already gave up (timed out or cancelled): corr_id={} topic={}",
        corr_id,
        topic
    );
}

#[cfg(feature = "channel")]
fn try_send_result(
    result: Result<bool, kanal::SendError>,
    corr_id: &str,
    topic: &str,
) -> ReplyOutcome {
    match result {
        Ok(true) => {
            tracing::debug!("resp sended corrid: {}", corr_id);
            ReplyOutcome::Delivered
        }
        Ok(false) => {
            tracing::error!(
                "reply channel full, reply discarded: corr_id={} topic={}",
                corr_id,
                topic
            );
            ReplyOutcome::ChannelFull
        }
        Err(_) => {
            log_late_reply(corr_id, topic);
            ReplyOutcome::LateReply
        }
    }
}

/// Abandons its own pending async request when dropped, so a
/// `send_request_async` future that is cancelled (for example by an outer
/// `tokio::time::timeout`) does not leave its entry behind. Does nothing if
/// the reply was already delivered or the corr id now belongs to another
/// request.
#[cfg(all(feature = "channel", feature = "tokio"))]
struct PendingAsyncGuard<'a> {
    inner: &'a SolClientInner,
    corr_id: String,
    id: u64,
}

#[cfg(all(feature = "channel", feature = "tokio"))]
impl Drop for PendingAsyncGuard<'_> {
    fn drop(&mut self) {
        self.inner.abandon_async_if(&self.corr_id, self.id);
    }
}
// #[cfg_attr(feature = "tokio", derive(Debug, Clone))]

#[derive(Debug, Snafu, PartialEq)]
pub enum SolClientError {
    #[snafu(display("SolClient context create Error"))]
    ContextCreate,
    #[snafu(display("SolClient send request {topic}, code: {code:?}, Error {error:?}"))]
    SendRequest {
        topic: String,
        code: SolClientReturnCode,
        error: ErrorInfo,
    },
    #[snafu(display("SolClient send cache request {topic}, request_id: {request_id}, code: {code:?}, Error {error:?}"))]
    SendCacheRequest {
        topic: String,
        request_id: u64,
        code: SolClientReturnCode,
        error: ErrorInfo,
    },
    #[snafu(display("SolClient inside {}", source))]
    SolMsg { source: SolMsgError },
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolClientRxCallbackInfo {
    pub callback_p: rsolace_sys::solClient_flow_rxMsgCallbackFunc_t,
    // pub user_p: i32,
    pub user_p: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolClinetEventCallbackInfo {
    pub callback_p: rsolace_sys::solClient_session_eventCallbackFunc_t,
    // pub user_p: i32,
    pub user_p: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SolClientRxMsgCallbackInfo {
    pub callback_p: rsolace_sys::solClient_flow_rxMsgCallbackFunc_t,
    // pub user_p: i32,
    pub user_p: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct SolClientFuncInfo {
    pub rxInfo: SolClientRxCallbackInfo,
    pub eventInfo: SolClinetEventCallbackInfo,
    pub rxMsgInfo: SolClientRxMsgCallbackInfo,
}

impl From<SolClientFuncInfo> for rsolace_sys::solClient_session_createFuncInfo_t {
    fn from(value: SolClientFuncInfo) -> Self {
        Self {
            rxMsgInfo: rsolace_sys::solClient_session_createRxMsgCallbackFuncInfo_t {
                callback_p: value.rxMsgInfo.callback_p,
                // user_p: value.rxMsgInfo.user_p as *mut _,
                user_p: value.rxMsgInfo.user_p,
            },
            eventInfo: rsolace_sys::solClient_session_createEventCallbackFuncInfo_t {
                callback_p: value.eventInfo.callback_p,
                // user_p: value.eventInfo.user_p as *mut _,
                user_p: value.eventInfo.user_p,
            },
            rxInfo: rsolace_sys::solClient_session_createRxCallbackFuncInfo {
                callback_p: null_mut(),
                // user_p: value.rxInfo.user_p as *mut _,
                user_p: value.rxInfo.user_p,
            },
        }
    }
}

struct SolClientInner {
    // context_p: i32,
    context_p: rsolace_sys::solClient_opaqueContext_pt,
    // context_func_info: rsolace_sys::solClient_context_createFuncInfo_t,
    // session_p: rsolace_sys::solClient_opaqueSession_pt,
    // session_p: Option<i32>,
    // session_p: i32,
    session_p: rsolace_sys::solClient_opaqueSession_pt,
    // session_func_info: Option<rsolace_sys::solClient_session_createFuncInfo_t>,
    // session_func_info: Option<i32>,
    session_func_info: Option<SolClientFuncInfo>,
    #[cfg(feature = "raw")]
    rx_msg_callback: Option<fn(&mut Self, SolMsg)>,
    #[cfg(feature = "raw")]
    rx_event_callback: Option<fn(&mut Self, SolEvent)>,
    #[cfg(feature = "channel")]
    msg_sender: Sender<SolMsg>,
    #[cfg(feature = "channel")]
    msg_receiver: Receiver<SolMsg>,
    #[cfg(feature = "channel")]
    p2p_sender: Sender<SolMsg>,
    #[cfg(feature = "channel")]
    p2p_receiver: Receiver<SolMsg>,
    #[cfg(feature = "channel")]
    request_sender: Sender<SolMsg>,
    #[cfg(feature = "channel")]
    request_receiver: Receiver<SolMsg>,
    #[cfg(feature = "channel")]
    event_sender: Sender<SolEvent>,
    #[cfg(feature = "channel")]
    event_receiver: Receiver<SolEvent>,
    #[cfg(feature = "channel")]
    request_reply_map: DashMap<String, PendingSyncRequest>,
    #[cfg(all(feature = "channel", feature = "tokio"))]
    async_request_reply_map: DashMap<String, PendingAsyncRequest>,
    /// Unique id for each request put in a reply map.
    #[cfg(feature = "channel")]
    next_request_id: AtomicU64,
    /// Requests whose caller gave up, so a late reply logs with its topic.
    #[cfg(feature = "channel")]
    abandoned_requests: AbandonedRequests,
    /// Monotonic counter used to generate unique correlation tags for
    /// non-blocking session-modify operations. Starts at 1 so a `null`
    /// correlation pointer is always distinguishable from a valid tag.
    #[cfg(feature = "channel")]
    next_modify_prop_tag: AtomicUsize,
    /// One-shot waiters keyed by correlation tag for in-flight
    /// `solClient_session_modifyClientInfo` calls. The event callback
    /// resolves the matching waiter with the `ModifyPropOk` /
    /// `ModifyPropFail` outcome reported by the C API.
    #[cfg(feature = "channel")]
    modify_prop_waiters: DashMap<usize, Sender<SolClientSessionEvent>>,
}

#[cfg(feature = "channel")]
impl SolClientInner {
    fn next_request_id(&self) -> u64 {
        self.next_request_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Route an incoming reply to its waiting requester, or log it as a late
    /// reply if the requester already gave up.
    fn dispatch_reply(&self, corr_id: &str, msg: SolMsg) -> ReplyOutcome {
        if let Some((_, pending)) = self.request_reply_map.remove(corr_id) {
            return try_send_result(pending.sender.try_send(msg), corr_id, &pending.topic);
        }
        #[cfg(feature = "tokio")]
        if let Some((_, pending)) = self.async_request_reply_map.remove(corr_id) {
            return try_send_result(pending.sender.try_send(msg), corr_id, &pending.topic);
        }
        // Checked under the abandoned lock, which cancel holds while moving a
        // request from pending to abandoned, so a reply racing a cancel sees
        // either the pending entry or the abandoned record. No sender is sent
        // to or dropped while that lock is held.
        let abandoned_topic = self.abandoned_requests.lock().take(corr_id);
        if let Some(topic) = abandoned_topic {
            log_late_reply(corr_id, &topic);
            return ReplyOutcome::LateReply;
        }
        tracing::debug!("reply with no pending request: corr_id={}", corr_id);
        ReplyOutcome::Unmatched
    }

    /// Remove whatever request is pending under `corr_id` and remember it as
    /// abandoned.
    fn cancel(&self, corr_id: &str) -> bool {
        // Senders removed under the lock are dropped only after it is
        // released: dropping a kanal sender can wake a task synchronously, and
        // that task must be free to call back into cancel_request.
        let mut released_sync: Option<Sender<SolMsg>> = None;
        #[cfg(feature = "tokio")]
        let mut released_async: Option<AsyncSender<SolMsg>> = None;
        {
            // Hold the abandoned lock across remove + record (see dispatch_reply).
            let mut abandoned = self.abandoned_requests.lock();
            if let Some((k, p)) = self.request_reply_map.remove(corr_id) {
                abandoned.record(k, p.topic);
                released_sync = Some(p.sender);
            }
            #[cfg(feature = "tokio")]
            if let Some((k, p)) = self.async_request_reply_map.remove(corr_id) {
                abandoned.record(k, p.topic);
                released_async = Some(p.sender);
            }
        }
        #[cfg(feature = "tokio")]
        let removed = released_sync.is_some() || released_async.is_some();
        #[cfg(not(feature = "tokio"))]
        let removed = released_sync.is_some();
        drop(released_sync);
        #[cfg(feature = "tokio")]
        drop(released_async);
        removed
    }

    /// Abandon the async request under `corr_id` only if it is still the
    /// request with this `id`. A completed request (entry gone) or a later
    /// request that reused the corr id is left alone.
    #[cfg(feature = "tokio")]
    fn abandon_async_if(&self, corr_id: &str, id: u64) -> bool {
        let released = {
            // Hold the abandoned lock across remove + record (see dispatch_reply).
            let mut abandoned = self.abandoned_requests.lock();
            self.async_request_reply_map
                .remove_if(corr_id, |_, p| p.id == id)
                .map(|(k, p)| {
                    abandoned.record(k, p.topic);
                    p.sender
                })
        };
        // Dropped after the lock is released (see cancel).
        released.is_some()
    }
}

pub struct SolClient {
    inner: Pin<Box<SolClientInner>>,
}

impl Default for SolClient {
    fn default() -> Self {
        Self::new(SolClientLogLevel::Notice).unwrap()
    }
}

impl SolClient {
    // Helper method to access inner safely
    fn inner(&self) -> &SolClientInner {
        &*self.inner
    }

    fn inner_mut(&mut self) -> &mut SolClientInner {
        self.inner.as_mut().get_mut()
    }

    /// Cleanly disconnect and destroy the current session, if any.
    /// After this call, `session_p` is null and safe to overwrite.
    fn destroy_session(&mut self) {
        unsafe {
            if !self.inner().session_p.is_null() {
                rsolace_sys::solClient_session_disconnect(self.inner().session_p);
                let mut session_p = self.inner().session_p;
                rsolace_sys::solClient_session_destroy(&mut session_p);
                self.inner_mut().session_p = std::ptr::null_mut();
                self.inner_mut().session_func_info = None;
            }
        }
        // Clear pending request/reply maps to unblock any waiting callers
        #[cfg(feature = "channel")]
        self.inner().request_reply_map.clear();
        #[cfg(all(feature = "channel", feature = "tokio"))]
        self.inner().async_request_reply_map.clear();
        #[cfg(feature = "channel")]
        self.inner().abandoned_requests.clear();
        // Drop any pending modify_client_info waiters. Dropping the senders
        // closes the channels so `recv_timeout` returns immediately instead
        // of blocking the full 10s when the session is torn down.
        #[cfg(feature = "channel")]
        self.inner().modify_prop_waiters.clear();
    }

    pub fn new(log_level: SolClientLogLevel) -> Result<SolClient, SolClientError> {
        let mut context_p: rsolace_sys::solClient_opaqueContext_pt = null_mut();
        let session_p: rsolace_sys::solClient_opaqueSession_pt = null_mut();
        unsafe {
            rsolace_sys::solClient_initialize(
                log_level as rsolace_sys::solClient_log_level_t,
                null_mut(),
            );
            let nullptr: *mut std::ffi::c_void = null_mut();
            let mut conext_props: [*const c_char; 3] = [
                rsolace_sys::SOLCLIENT_CONTEXT_PROP_CREATE_THREAD.as_ptr() as *const c_char,
                rsolace_sys::SOLCLIENT_PROP_ENABLE_VAL.as_ptr() as *const c_char,
                null(),
            ];
            let conext_props_ptr: rsolace_sys::solClient_propertyArray_pt =
                conext_props.as_mut_ptr();

            let mut context_func_info: rsolace_sys::solClient_context_createFuncInfo_t =
                rsolace_sys::solClient_context_createFuncInfo {
                    regFdInfo: rsolace_sys::solClient_context_createRegisterFdFuncInfo {
                        regFdFunc_p: None,
                        unregFdFunc_p: None,
                        user_p: nullptr,
                    },
                };
            tracing::debug!("context_p: {:?}", context_p);
            let rt_code = rsolace_sys::solClient_context_create(
                conext_props_ptr,
                &mut context_p,
                &mut context_func_info,
                std::mem::size_of::<rsolace_sys::solClient_context_createFuncInfo>(),
            );
            tracing::debug!("context_p: {:?}", context_p);
            ensure!(
                rt_code == rsolace_sys::solClient_returnCode_SOLCLIENT_OK,
                ContextCreateSnafu
            );

            let (msg_sender, msg_receiver) = unbounded();
            let (p2p_sender, p2p_receiver) = unbounded();
            let (request_sender, request_receiver) = unbounded();
            let (event_sender, envent_receiver) = unbounded();

            let inner = SolClientInner {
                // context_p: context_p as i32,
                context_p,
                // context_func_info: context_func_info,
                // session_p: session_p as i32,
                session_p,
                session_func_info: None,
                #[cfg(feature = "raw")]
                rx_msg_callback: None,
                #[cfg(feature = "raw")]
                rx_event_callback: None,
                #[cfg(feature = "channel")]
                msg_sender,
                #[cfg(feature = "channel")]
                msg_receiver,
                #[cfg(feature = "channel")]
                p2p_sender,
                #[cfg(feature = "channel")]
                p2p_receiver,
                #[cfg(feature = "channel")]
                request_sender,
                #[cfg(feature = "channel")]
                request_receiver,
                #[cfg(feature = "channel")]
                event_sender,
                #[cfg(feature = "channel")]
                event_receiver: envent_receiver,
                #[cfg(feature = "channel")]
                // request_reply_map: Arc::new(DashMap::new()),
                request_reply_map: DashMap::new(),
                #[cfg(all(feature = "channel", feature = "tokio"))]
                // async_request_reply_map: Arc::new(DashMap::new()),
                async_request_reply_map: DashMap::new(),
                #[cfg(feature = "channel")]
                next_request_id: AtomicU64::new(1),
                #[cfg(feature = "channel")]
                abandoned_requests: AbandonedRequests::default(),
                #[cfg(feature = "channel")]
                // Start at 1: 0 would round-trip to a null pointer and be
                // indistinguishable from "no correlation supplied".
                next_modify_prop_tag: AtomicUsize::new(1),
                #[cfg(feature = "channel")]
                modify_prop_waiters: DashMap::new(),
            };

            Ok(SolClient {
                inner: Box::pin(inner),
            })
        }
    }

    pub fn connect(&mut self, props: SessionProps) -> bool {
        // Clean up any existing session before creating a new one.
        // This prevents the old session's background cleanup from
        // interfering with the new session's channels. (Issue #6)
        self.destroy_session();

        let mut session_props = props.to_c();
        let session_props_ptr: rsolace_sys::solClient_propertyArray_pt = session_props.as_mut_ptr();

        // Use stable pointer from Pin<Box> - this is guaranteed never to move
        let user_p: *mut c_void = self.inner_mut() as *mut _ as *mut c_void;
        tracing::debug!("connect user_p: {:?}", user_p);

        unsafe extern "C" fn message_receive_callback(
            _opaque_session_p: rsolace_sys::solClient_opaqueSession_pt,
            msg_p: rsolace_sys::solClient_opaqueMsg_pt,
            user_p: *mut std::ffi::c_void,
        ) -> rsolace_sys::solClient_rxMsgCallback_returnCode_t {
            let solmsg = SolMsg::from_ptr(msg_p);
            match solmsg {
                Ok(msg) => {
                    let self_ref: &SolClientInner = &*(user_p as *const SolClientInner);
                    tracing::debug!("user_p: {:?}", user_p);
                    #[cfg(feature = "channel")]
                    {
                        if msg.is_reply() {
                            let corr_id = msg.get_correlation_id().unwrap();
                            tracing::debug!("resp msg corrid: {}", corr_id);
                            self_ref.dispatch_reply(&corr_id, msg);
                        } else {
                            match msg.get_reply_to() {
                                Ok(reply_to) => {
                                    tracing::debug!("msg reply to: {:?}", reply_to);
                                    if let Err(e) = self_ref.request_sender.send(msg) {
                                        tracing::error!("send request msg to channel error: {}", e);
                                    }
                                }
                                Err(_e) => {
                                    if msg.is_p2p() {
                                        if let Err(e) = self_ref.p2p_sender.send(msg) {
                                            tracing::error!("send p2p msg to channel error: {}", e);
                                        }
                                    } else if let Err(e) = self_ref.msg_sender.send(msg) {
                                        tracing::error!("send msg to channel error: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    #[cfg(feature = "raw")]
                    {
                        if let Some(cb) = self_ref.rx_msg_callback {
                            cb(self_ref, msg);
                        } else {
                            msg.dump(true);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("msg from ptr error: {}", e);
                }
            }
            rsolace_sys::solClient_rxMsgCallback_returnCode_SOLCLIENT_CALLBACK_TAKE_MSG
        }

        unsafe extern "C" fn event_receive_callback(
            _opaque_session_p: rsolace_sys::solClient_opaqueSession_pt,
            event_info_p: rsolace_sys::solClient_session_eventCallbackInfo_pt,
            user_p: *mut std::ffi::c_void,
        ) {
            let event = SolEvent::from_ptr(event_info_p);
            match event {
                Ok(event) => {
                    let self_ref: &SolClientInner = &*(user_p as *const SolClientInner);
                    #[cfg(feature = "raw")]
                    {
                        if let Some(cb) = self_ref.rx_event_callback {
                            cb(self_ref, event.clone())
                        } else {
                            tracing::info!(
                                "event: {}, response code: {}, info: {}",
                                event.get_session_event_string(),
                                event.response_code,
                                event.info
                            );
                        }
                    }
                    #[cfg(feature = "channel")]
                    {
                        // Dispatch ModifyPropOk / ModifyPropFail to any
                        // in-flight modify_client_info waiter keyed by the
                        // correlation tag we supplied. Always still forward
                        // the event downstream so observers on the public
                        // event channel can see modify-prop outcomes too.
                        if matches!(
                            event.session_event,
                            SolClientSessionEvent::ModifyPropOk
                                | SolClientSessionEvent::ModifyPropFail
                        ) {
                            if let Some(tag) = event.correlation_tag {
                                if let Some((_tag, sender)) =
                                    self_ref.modify_prop_waiters.remove(&tag)
                                {
                                    // bounded(1): try_send avoids any
                                    // possibility of blocking the C API
                                    // context thread if the waiter has gone
                                    // away.
                                    if let Err(e) = sender.try_send(event.session_event) {
                                        tracing::error!(
                                            "modify_client_info waiter try_send error: {:?}",
                                            e
                                        );
                                    }
                                }
                            }
                        }
                        if let Err(e) = self_ref.event_sender.send(event) {
                            tracing::error!("send event to channel error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("event from ptr error: {}", e);
                }
            }
        }

        self.inner_mut().session_func_info = Some(SolClientFuncInfo {
            rxInfo: SolClientRxCallbackInfo {
                callback_p: None,
                // user_p: user_p as i32,
                user_p,
            },
            eventInfo: SolClinetEventCallbackInfo {
                callback_p: Some(event_receive_callback),
                // user_p: user_p as i32,
                user_p,
            },
            rxMsgInfo: SolClientRxMsgCallbackInfo {
                callback_p: Some(message_receive_callback),
                // user_p: user_p as i32,
                user_p,
            },
        });
        let session_func_info = Some(rsolace_sys::solClient_session_createFuncInfo_t {
            rxMsgInfo: rsolace_sys::solClient_session_createRxMsgCallbackFuncInfo_t {
                callback_p: Some(message_receive_callback),
                user_p,
            },
            eventInfo: rsolace_sys::solClient_session_createEventCallbackFuncInfo_t {
                callback_p: Some(event_receive_callback),
                user_p,
            },
            rxInfo: rsolace_sys::solClient_session_createRxCallbackFuncInfo {
                callback_p: null_mut(),
                user_p,
            },
        });
        let session_func_info_ptr: *mut rsolace_sys::solClient_session_createFuncInfo_t =
            &mut session_func_info.unwrap();
        // &mut (rsolace_sys::solClient_session_createFuncInfo_t::from(self.session_func_info.unwrap()));
        let mut session_p: rsolace_sys::solClient_opaqueSession_pt = null_mut();
        unsafe {
            rsolace_sys::solClient_session_create(
                session_props_ptr,
                self.inner().context_p,
                &mut session_p,
                session_func_info_ptr,
                std::mem::size_of::<rsolace_sys::solClient_session_createFuncInfo>(),
            );
            // self.inner_mut().session_p = session_p as i32;
            self.inner_mut().session_p = session_p;
            let rt_code = rsolace_sys::solClient_session_connect(self.inner().session_p);
            rt_code == (SolClientReturnCode::Ok as i32)
        }
    }

    pub fn disconnect(&self) {
        let user_p: *const c_void = self.inner() as *const _ as *const c_void;
        tracing::debug!("disconnect const user_p: {:?}", user_p);
        unsafe {
            // if self.inner().session_p != 0 {
            if !self.inner().session_p.is_null() {
                // Just disconnect the session - destroy will be handled in Drop
                rsolace_sys::solClient_session_disconnect(self.inner().session_p);
            }
        }
    }

    #[cfg(feature = "raw")]
    pub fn set_rx_msg_callback(&mut self, func: fn(&mut SolClientInner, SolMsg)) {
        self.inner_mut().rx_msg_callback = Some(func);
    }

    #[cfg(feature = "raw")]
    pub fn set_rx_event_callback(&mut self, func: fn(&mut SolClientInner, SolEvent)) {
        self.inner_mut().rx_event_callback = Some(func);
    }

    #[cfg(feature = "channel")]
    pub fn get_msg_receiver(&self) -> Receiver<SolMsg> {
        self.inner().msg_receiver.clone()
    }

    #[cfg(feature = "channel")]
    pub fn get_request_receiver(&self) -> Receiver<SolMsg> {
        self.inner().request_receiver.clone()
    }

    #[cfg(feature = "channel")]
    pub fn get_p2p_receiver(&self) -> Receiver<SolMsg> {
        self.inner().p2p_receiver.clone()
    }

    #[cfg(feature = "channel")]
    pub fn get_event_receiver(&self) -> Receiver<SolEvent> {
        self.inner().event_receiver.clone()
    }

    // Async channel support with kanal
    #[cfg(all(feature = "channel", feature = "tokio"))]
    pub fn get_async_msg_receiver(&self) -> AsyncReceiver<SolMsg> {
        self.inner().msg_receiver.as_async().clone()
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    pub fn get_async_request_receiver(&self) -> AsyncReceiver<SolMsg> {
        self.inner().request_receiver.as_async().clone()
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    pub fn get_async_p2p_receiver(&self) -> AsyncReceiver<SolMsg> {
        self.inner().p2p_receiver.as_async().clone()
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    pub fn get_async_event_receiver(&self) -> AsyncReceiver<SolEvent> {
        self.inner().event_receiver.as_async().clone()
    }

    pub fn subscribe(&self, topic: &str) -> SolClientReturnCode {
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code = rsolace_sys::solClient_session_topicSubscribe(
                self.inner().session_p,
                topic.as_ptr(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn unsubscribe(&self, topic: &str) -> SolClientReturnCode {
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code = rsolace_sys::solClient_session_topicUnsubscribe(
                self.inner().session_p,
                topic.as_ptr(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn subscribe_ext(&self, topic: &str, flag: SolClientSubscribeFlags) -> SolClientReturnCode {
        let user_p: *const c_void = self.inner() as *const _ as *const c_void;
        tracing::debug!("subscribe const user_p: {:?}", user_p);
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code = rsolace_sys::solClient_session_topicSubscribeExt(
                self.inner().session_p,
                flag as rsolace_sys::solClient_subscribeFlags_t,
                topic.as_ptr(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn unsubscribe_ext(
        &self,
        topic: &str,
        flag: SolClientSubscribeFlags,
    ) -> SolClientReturnCode {
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code = rsolace_sys::solClient_session_topicUnsubscribeExt(
                self.inner().session_p,
                flag as rsolace_sys::solClient_subscribeFlags_t,
                topic.as_ptr(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn send_msg(&self, msg: &SolMsg) -> SolClientReturnCode {
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendMsg(self.inner().session_p, msg.get_ptr())
        };
        SolClientReturnCode::from_i32(rt_code).unwrap()
    }

    /// Sends the messages with one `solClient_session_sendMultipleMsg` call. Prefer
    /// [`Self::send_multiple_msg_written`], which also reports how many were accepted.
    ///
    /// More than [`SEND_MULTIPLE_LIMIT`] messages are refused with `Fail` (nothing sent).
    pub fn send_multiple_msg(&self, msgs: &[&SolMsg]) -> SolClientReturnCode {
        self.send_multiple_msg_written(msgs).0
    }

    /// Sends up to [`SEND_MULTIPLE_LIMIT`] messages with one
    /// `solClient_session_sendMultipleMsg` call and returns the return code together with
    /// the number of messages the API accepted (`numberOfMessagesWritten`).
    ///
    /// The accepted messages are always a prefix of `msgs`. The count equals `msgs.len()`
    /// when the code is `Ok`; on any other code (`WouldBlock`, `NotReady`, `Fail`) part of
    /// the slice may have been written, and only `msgs[written..]` must be resent or
    /// dropped by the caller.
    ///
    /// An empty slice returns `(Ok, 0)` and more than [`SEND_MULTIPLE_LIMIT`] messages
    /// return `(Fail, 0)`; neither calls the API. The messages stay owned by the caller and
    /// are freed when the `SolMsg`s are dropped.
    pub fn send_multiple_msg_written(&self, msgs: &[&SolMsg]) -> (SolClientReturnCode, usize) {
        if msgs.is_empty() {
            return (SolClientReturnCode::Ok, 0);
        }
        if msgs.len() > SEND_MULTIPLE_LIMIT {
            return (SolClientReturnCode::Fail, 0);
        }
        let mut arr_msg: [rsolace_sys::solClient_opaqueMsg_pt; SEND_MULTIPLE_LIMIT] =
            [null_mut(); SEND_MULTIPLE_LIMIT];
        for (slot, msg) in arr_msg.iter_mut().zip(msgs) {
            *slot = msg.get_ptr();
        }
        let mut written: rsolace_sys::solClient_uint32_t = 0;
        // SAFETY: the first `msgs.len()` (<= SEND_MULTIPLE_LIMIT) entries of `arr_msg` are
        // valid message pointers borrowed from `msgs` for the duration of the call; the API
        // copies what it sends and does not keep the pointers.
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendMultipleMsg(
                self.inner().session_p,
                arr_msg.as_mut_ptr(),
                msgs.len() as rsolace_sys::solClient_uint32_t,
                &mut written,
            )
        };
        let code = SolClientReturnCode::from_i32(rt_code).unwrap_or(SolClientReturnCode::Fail);
        // Never report more than was offered (the API guarantees it; keep slicing safe).
        (code, (written as usize).min(msgs.len()))
    }

    fn send_request_unsafe_part(
        &self,
        msg: &SolMsg,
        timeout: u32,
    ) -> (SolClientReturnCode, rsolace_sys::solClient_opaqueMsg_pt) {
        let mut reply_msg_pt: rsolace_sys::solClient_opaqueMsg_pt = null_mut();
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendRequest(
                self.inner().session_p,
                msg.get_ptr(),
                &mut reply_msg_pt,
                timeout,
            )
        };
        (
            SolClientReturnCode::from_i32(rt_code).unwrap(),
            reply_msg_pt,
        )
    }

    pub fn get_last_error_info(&self) -> Option<ErrorInfo> {
        let error_info_ptr = unsafe { rsolace_sys::solClient_getLastErrorInfo() };
        let error_info = ErrorInfo::from_error_info_ptr(error_info_ptr);
        unsafe { rsolace_sys::solClient_resetLastErrorInfo() };
        error_info
    }

    #[cfg(feature = "raw")]
    pub fn send_request(&self, msg: &SolMsg, timeout: u32) -> Result<SolMsg, SolClientError> {
        let (rt_code, reply_msg_pt) = self.send_request_unsafe_part(msg, timeout);

        ensure!(
            (timeout > 0 && rt_code == SolClientReturnCode::Ok)
                || (timeout == 0 && rt_code == SolClientReturnCode::InProgress),
            SendRequestSnafu {
                topic: msg.get_topic().context(SolMsgSnafu)?,
                code: rt_code,
                error: self.get_last_error_info().unwrap(),
            }
        );
        // check reply msg when non block
        Ok(SolMsg::from_ptr(reply_msg_pt).unwrap())
    }

    #[cfg(feature = "channel")]
    pub fn send_request(
        &self,
        msg: &SolMsg,
        timeout: u32,
    ) -> Result<Receiver<SolMsg>, SolClientError> {
        // tracing::debug!("send request with channel, corrid: {}", corrid);
        let (s, r) = bounded(1);
        // tracing::debug!(
        //     "send request with channel, corrid: {}, insert to map",
        //     corrid
        // );
        // let reply_msg_pt: rsolace_sys::solClient_opaqueMsg_pt = null_mut();
        if timeout == 0 {
            let corrid = msg.get_correlation_id().unwrap_or("c0".into());
            let id = self.inner().next_request_id();
            let topic = msg.get_topic().unwrap_or_else(|_| "unknown".into());
            {
                self.inner().request_reply_map.insert(
                    corrid.clone(),
                    PendingSyncRequest {
                        id,
                        topic,
                        sender: s,
                    },
                );
            }
            // tracing::debug!("send request with channel insert to map done");
            let (rt_code, _) = self.send_request_unsafe_part(msg, timeout);
            if rt_code != SolClientReturnCode::InProgress {
                // No reply will ever come for a request that was not sent.
                self.inner()
                    .request_reply_map
                    .remove_if(&corrid, |_, p| p.id == id);
            }
            ensure!(
                rt_code == SolClientReturnCode::InProgress,
                SendRequestSnafu {
                    topic: msg.get_topic().context(SolMsgSnafu)?,
                    code: rt_code,
                    error: self.get_last_error_info().unwrap(),
                }
            );
        } else {
            let (rt_code, reply_msg_pt) = self.send_request_unsafe_part(msg, timeout);
            ensure!(
                rt_code == SolClientReturnCode::Ok,
                SendRequestSnafu {
                    topic: msg.get_topic().context(SolMsgSnafu)?,
                    code: rt_code,
                    error: self.get_last_error_info().unwrap(),
                }
            );
            if rt_code == SolClientReturnCode::Ok {
                s.send(unsafe { SolMsg::from_ptr(reply_msg_pt).unwrap() })
                    .unwrap();
            }
        }
        Ok(r)
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    pub fn send_request_async_receiver(
        &self,
        msg: &SolMsg,
    ) -> Result<AsyncReceiver<SolMsg>, SolClientError> {
        self.send_request_async_tracked(msg).map(|(r, _, _)| r)
    }

    /// Like [`SolClient::send_request_async_receiver`], also returning the
    /// corr id and the unique id of the pending entry.
    #[cfg(all(feature = "channel", feature = "tokio"))]
    fn send_request_async_tracked(
        &self,
        msg: &SolMsg,
    ) -> Result<(AsyncReceiver<SolMsg>, String, u64), SolClientError> {
        // let user_p: *const c_void = self.inner() as *const _ as *const c_void;
        // tracing::debug!("send_request_async_receiver const user_p: {:?}", user_p);
        // For async implementation, we'll use kanal's async bounded channel
        let (s, r) = bounded_async(1);

        let corrid = msg.get_correlation_id().unwrap_or("c0".into());
        let topic = msg.get_topic().unwrap_or_else(|_| "unknown".into());
        let id = self.inner().next_request_id();
        // Store the async sender (with its topic, for logging) in the map
        // before sending, so a fast reply always finds it.
        {
            self.inner().async_request_reply_map.insert(
                corrid.clone(),
                PendingAsyncRequest {
                    id,
                    topic,
                    sender: s,
                },
            );
        }
        let (rt_code, _) = self.send_request_unsafe_part(msg, 0);
        if rt_code != SolClientReturnCode::InProgress {
            // No reply will ever come for a request that was not sent.
            self.inner()
                .async_request_reply_map
                .remove_if(&corrid, |_, p| p.id == id);
        }
        ensure!(
            rt_code == SolClientReturnCode::InProgress,
            SendRequestSnafu {
                topic: msg.get_topic().context(SolMsgSnafu)?,
                code: rt_code,
                error: self.get_last_error_info().unwrap(),
            }
        );
        Ok((r, corrid, id))
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    pub async fn send_request_async(&self, msg: &SolMsg) -> Result<SolMsg, SolClientError> {
        let topic = msg.get_topic().context(SolMsgSnafu)?;
        let (receiver, corr_id, id) = self.send_request_async_tracked(msg)?;
        // If this future is dropped before the reply arrives (outer timeout,
        // select!, task abort), abandon its own pending entry with it.
        let _guard = PendingAsyncGuard {
            inner: self.inner(),
            corr_id,
            id,
        };
        receiver
            .recv()
            .await
            .map_err(|_| SolClientError::SendRequest {
                topic: topic,
                code: SolClientReturnCode::Fail,
                error: self.get_last_error_info().unwrap_or_else(|| {
                    use crate::types::{ErrorInfo, SolClientSubCodeOrRaw};
                    ErrorInfo {
                        sub_code: SolClientSubCodeOrRaw::Raw(
                            SolClientReturnCode::Fail as rsolace_sys::solClient_subCode,
                        ),
                        error_str: "Response timeout or channel closed".to_string(),
                    }
                }),
            })
    }

    /// Stop waiting for a pending request by correlation id.
    ///
    /// Call this when the caller stops waiting for a reply obtained from
    /// [`SolClient::send_request_async_receiver`] or
    /// [`SolClient::send_request`] with `timeout == 0` (for example after its
    /// own timeout fires). The waiting entry is removed so it does not stay in
    /// memory if the reply never comes. The request is remembered for a while
    /// (bounded by count and age), so a reply that still arrives is logged at
    /// WARN as a late reply with its corr id and topic.
    ///
    /// Returns `true` if a pending entry was removed.
    #[cfg(feature = "channel")]
    pub fn cancel_request(&self, corr_id: &str) -> bool {
        self.inner().cancel(corr_id)
    }

    pub fn send_cache_request(
        &self,
        topic: &str,
        request_id: u64,
        props: CacheSessionProps,
        flags: SolClientCacheRequestFlags,
    ) -> Result<(), SolClientError> {
        let topic_c = topic.to_cstring();
        let cache_session_props_arr = props.to_c();
        let cache_session_props_ptr: *const *const c_char = cache_session_props_arr.as_ptr();
        let mut cache_session_ptr: rsolace_sys::solClient_opaqueCacheSession_pt = null_mut();
        let r = unsafe {
            rsolace_sys::solClient_session_createCacheSession(
                cache_session_props_ptr,
                self.inner().session_p,
                &mut cache_session_ptr,
            )
        };
        let res = SolClientReturnCode::from_i32(r).unwrap();
        ensure!(
            res == SolClientReturnCode::Ok,
            SendCacheRequestSnafu {
                topic: topic.to_string(),
                request_id,
                code: res,
                error: self.get_last_error_info().unwrap(),
            }
        );
        let callback_p: Option<
            unsafe extern "C" fn(
                *mut std::ffi::c_void,
                *mut rsolace_sys::solCache_eventCallbackInfo,
                *mut std::ffi::c_void,
            ),
        > = None;
        let rt_code = unsafe {
            rsolace_sys::solClient_cacheSession_sendCacheRequest(
                cache_session_ptr,
                topic_c.as_ptr(),
                request_id,
                callback_p,
                self.inner().session_p,
                flags as rsolace_sys::solClient_cacheRequestFlags_t,
                SolClientSubscribeFlags::RequestConfirm as rsolace_sys::solClient_subscribeFlags_t,
            )
        };
        let rt_code = SolClientReturnCode::from_i32(rt_code).unwrap();
        unsafe {
            rsolace_sys::solClient_cacheSession_destroy(&mut cache_session_ptr);
        }
        ensure!(
            rt_code == SolClientReturnCode::Ok,
            SendCacheRequestSnafu {
                topic: topic.to_string(),
                request_id,
                code: rt_code,
                error: self.get_last_error_info().unwrap(),
            }
        );
        Ok(())
    }

    pub fn send_reply(&self, rx_msg: &SolMsg, reply_msg: &SolMsg) -> SolClientReturnCode {
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendReply(
                self.inner().session_p,
                rx_msg.get_ptr(),
                reply_msg.get_ptr(),
            )
        };
        SolClientReturnCode::from_i32(rt_code).unwrap()
    }

    /// Modify the `APPLICATION_DESCRIPTION` and/or `CLIENT_NAME` of the
    /// current Session and synchronously wait for the broker confirmation.
    ///
    /// Internally this issues the modify in the C API's non-blocking
    /// mode (no `SOLCLIENT_MODIFYPROP_FLAGS_WAITFORCONFIRM`) with a
    /// per-call correlation tag, then awaits the matching
    /// `SOLCLIENT_SESSION_EVENT_MODIFYPROP_OK` /
    /// `SOLCLIENT_SESSION_EVENT_MODIFYPROP_FAIL` event before returning.
    ///
    /// The previous implementation used `WAITFORCONFIRM`, which the C API
    /// would consume internally: the public event callback never saw the
    /// MODIFYPROP_OK event, and the call returned as soon as the broker
    /// acknowledged receipt of the modify request, not when the in-API
    /// P2P-subscription migration had finished. That left a race window
    /// (~0-3 s) where the next `send_request` could time out because the
    /// new client name's reply-to subscription wasn't yet active.
    /// See <https://github.com/Yvictor/rsolace/issues/8>.
    ///
    /// Returns `SolClientReturnCode::Ok` on `ModifyPropOk`,
    /// `SolClientReturnCode::Fail` on `ModifyPropFail` or on internal
    /// timeout, and the C API's return code on any synchronous failure
    /// (e.g. the call was rejected before being dispatched).
    ///
    /// # Threading
    ///
    /// This call MUST NOT be invoked from within the Solace API context
    /// thread itself (e.g. from inside a message or event callback);
    /// doing so would deadlock because the event callback delivering the
    /// confirmation runs on that same context thread. rsolace's normal
    /// calling pattern (application thread / tokio runtime) is fine.
    pub fn modify_client_info(
        &mut self,
        app_description: Option<&str>,
        client_name: Option<&str>,
    ) -> SolClientReturnCode {
        // Keep the CStrings alive for the duration of the C call: the
        // raw pointers we hand to the C API are only valid while these
        // owners are in scope.
        let app_desc_c = app_description.map(|s| CString::new(s).unwrap());
        let client_name_c = client_name.map(|s| CString::new(s).unwrap());

        let mut client_info_props = Vec::<*const c_char>::new();

        if let Some(ref app_desc) = app_desc_c {
            client_info_props.push(
                rsolace_sys::SOLCLIENT_SESSION_PROP_APPLICATION_DESCRIPTION.as_ptr()
                    as *const c_char,
            );
            client_info_props.push(app_desc.as_ptr());
        }

        if let Some(ref name) = client_name_c {
            client_info_props
                .push(rsolace_sys::SOLCLIENT_SESSION_PROP_CLIENT_NAME.as_ptr() as *const c_char);
            client_info_props.push(name.as_ptr());
        }
        if client_info_props.is_empty() {
            return SolClientReturnCode::NotFound;
        }
        client_info_props.push(std::ptr::null());

        // Without the `channel` feature there is no waiter-map mechanism;
        // fall back to the previous blocking behaviour. Consumers without
        // the channel feature don't get the race fix, but they also
        // don't have the infrastructure for the async path.
        #[cfg(not(feature = "channel"))]
        {
            let rt_code = unsafe {
                rsolace_sys::solClient_session_modifyClientInfo(
                    self.inner().session_p,
                    client_info_props.as_mut_ptr(),
                    rsolace_sys::SOLCLIENT_MODIFYPROP_FLAGS_WAITFORCONFIRM,
                    std::ptr::null_mut(),
                )
            };
            return SolClientReturnCode::from_i32(rt_code).unwrap();
        }

        #[cfg(feature = "channel")]
        {
            // Allocate a unique correlation tag and register a one-shot
            // waiter BEFORE issuing the C call, so we never race the
            // event callback that may fire as soon as the call returns.
            let tag = self
                .inner()
                .next_modify_prop_tag
                .fetch_add(1, Ordering::Relaxed);
            let (sender, receiver) = bounded::<SolClientSessionEvent>(1);
            self.inner().modify_prop_waiters.insert(tag, sender);

            let rt_code = unsafe {
                rsolace_sys::solClient_session_modifyClientInfo(
                    self.inner().session_p,
                    client_info_props.as_mut_ptr(),
                    // Non-blocking mode: the C API will deliver the
                    // outcome via SOLCLIENT_SESSION_EVENT_MODIFYPROP_OK /
                    // MODIFYPROP_FAIL with our `correlation_p` returned
                    // verbatim. With WAITFORCONFIRM set, the C API would
                    // consume the confirmation internally and our event
                    // callback would never see it.
                    0,
                    tag as *mut c_void,
                )
            };
            let rt_code = SolClientReturnCode::from_i32(rt_code).unwrap();

            // The C API may return either OK or IN_PROGRESS to indicate
            // "request was queued, async confirmation will follow".
            // Anything else is a synchronous failure (e.g. NOT_READY,
            // WOULD_BLOCK, FAIL), so no event will arrive and we do not wait.
            if rt_code != SolClientReturnCode::Ok && rt_code != SolClientReturnCode::InProgress {
                self.inner().modify_prop_waiters.remove(&tag);
                return rt_code;
            }

            // Block until the matching MODIFYPROP_OK / _FAIL event fires
            // on the context thread, or we hit the documented Solace
            // default modify-prop timeout.
            let result =
                receiver.recv_timeout(Duration::from_millis(MODIFY_CLIENT_INFO_TIMEOUT_MS));
            // Ensure the waiter entry is gone on every path (timeout,
            // success, channel-closed). Drop is normally enough, but
            // belt-and-braces against the channel being resolved twice.
            self.inner().modify_prop_waiters.remove(&tag);

            match result {
                Ok(SolClientSessionEvent::ModifyPropOk) => SolClientReturnCode::Ok,
                Ok(SolClientSessionEvent::ModifyPropFail) => {
                    tracing::warn!(
                        "modify_client_info: broker returned MODIFYPROP_FAIL (tag={})",
                        tag
                    );
                    SolClientReturnCode::Fail
                }
                Ok(other) => {
                    // Should never happen: only OK/FAIL are dispatched.
                    tracing::error!(
                        "modify_client_info: unexpected event variant {:?} (tag={})",
                        other,
                        tag
                    );
                    SolClientReturnCode::Fail
                }
                Err(e) => {
                    tracing::warn!(
                        "modify_client_info: timed out or channel closed waiting for MODIFYPROP confirmation (tag={}, err={:?})",
                        tag,
                        e
                    );
                    SolClientReturnCode::Fail
                }
            }
        }
    }

    pub fn get_ptr(&self) -> *const c_void {
        self.inner() as *const _ as *const c_void
    }

    /// Convert to a thread-safe shared client wrapped in Arc.
    ///
    /// Use this for multi-threaded scenarios where multiple threads
    /// need to share access to the same client.
    ///
    /// # Example
    /// ```ignore
    /// let client = SolClient::new(SolClientLogLevel::Notice)?.into_shared();
    /// let client2 = client.clone();
    /// std::thread::spawn(move || {
    ///     client2.send_msg(&msg);
    /// });
    /// ```
    pub fn into_shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Drop for SolClient {
    fn drop(&mut self) {
        self.destroy_session();

        let mut context_p = self.inner().context_p;
        unsafe {
            rsolace_sys::solClient_context_destroy(&mut context_p);
            rsolace_sys::solClient_cleanup();
        }
        tracing::debug!("solace client dropped");
    }
}

// SAFETY: SolClient is Send + Sync because:
// 1. Solace C API guarantees that Session operations are thread-safe
//    (see: https://docs.solace.com/API/API-Developer-Guide/Thread-Safety.htm)
// 2. Internal state uses thread-safe types (DashMap, kanal::Sender/Receiver)
// 3. Pin<Box<SolClientInner>> ensures the callback user_p pointer remains valid
//    even after SolClient is moved to another thread
//
// For multi-threaded usage, wrap in Arc<SolClient>:
// ```
// let client = Arc::new(SolClient::new(SolClientLogLevel::Notice)?);
// let client2 = client.clone();
// std::thread::spawn(move || {
//     client2.send_msg(&msg);
// });
// ```
unsafe impl Send for SolClient {}
unsafe impl Sync for SolClient {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destroy_session_on_null_session() {
        let mut client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        // session_p starts null — destroy_session should be a no-op
        client.destroy_session();
    }

    #[test]
    fn test_destroy_session_idempotent() {
        let mut client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        client.destroy_session();
        client.destroy_session();
        // No panic or double-free
    }

    #[test]
    fn test_double_disconnect() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        client.disconnect();
        client.disconnect();
        // No panic — disconnect on null session is safe
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_channels_persist_after_destroy() {
        let mut client = SolClient::new(SolClientLogLevel::Notice).unwrap();

        let msg_recv = client.get_msg_receiver();
        let event_recv = client.get_event_receiver();
        let p2p_recv = client.get_p2p_receiver();
        let request_recv = client.get_request_receiver();

        client.destroy_session();

        // Senders should still be open — channels are created in new(), not connect()
        let msg = SolMsg::new().unwrap();
        assert!(client.inner().msg_sender.send(msg).is_ok());

        let event = SolEvent::new(crate::types::SolClientSessionEvent::UpNotice, 0, "test");
        assert!(client.inner().event_sender.send(event).is_ok());

        // Receivers should get the messages we just sent
        assert!(msg_recv.try_recv().is_ok());
        assert!(event_recv.try_recv().is_ok());

        // All receivers are still valid (not closed)
        drop(p2p_recv);
        drop(request_recv);
    }

    #[test]
    fn test_drop_after_disconnect() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        client.disconnect();
        drop(client);
        // No double-free or panic
    }

    #[test]
    fn test_connect_twice_without_disconnect() {
        let mut client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        // First connect with unreachable host — session is created but connect fails
        let props1 = SessionProps::default()
            .host("tcp://127.0.0.1:1")
            .vpn("default")
            .username("test")
            .password("test")
            .connect_timeout_ms(100)
            .connect_retries(0);
        let _ = client.connect(props1);
        // Second connect — destroy_session() at start should clean up the first
        let props2 = SessionProps::default()
            .host("tcp://127.0.0.1:1")
            .vpn("default")
            .username("test")
            .password("test")
            .connect_timeout_ms(100)
            .connect_retries(0);
        let _ = client.connect(props2);
        // No panic, no leak, no double-free
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_request_reply_map_cleared_on_destroy() {
        let mut client = SolClient::new(SolClientLogLevel::Notice).unwrap();

        // Insert a dummy entry into request_reply_map
        let _r = insert_sync(&client, "test-corr-id", 1);
        client
            .inner()
            .abandoned_requests
            .lock()
            .record("test-abandoned".into(), TOPIC.into());
        assert!(!client.inner().request_reply_map.is_empty());

        client.destroy_session();

        assert!(client.inner().request_reply_map.is_empty());
        assert_eq!(client.inner().abandoned_requests.len(), 0);
    }

    #[cfg(feature = "channel")]
    #[derive(Clone, Default)]
    struct LogBuf(Arc<Mutex<Vec<u8>>>);

    #[cfg(feature = "channel")]
    impl std::io::Write for LogBuf {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    /// Run `f` with a WARN-level subscriber and return what it logged.
    #[cfg(feature = "channel")]
    fn capture_logs<R>(f: impl FnOnce() -> R) -> (R, String) {
        let buf = LogBuf::default();
        let writer = buf.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .with_ansi(false)
            .with_writer(move || writer.clone())
            .finish();
        let r = tracing::subscriber::with_default(subscriber, f);
        let out = String::from_utf8(buf.0.lock().unwrap().clone()).unwrap();
        (r, out)
    }

    #[cfg(feature = "channel")]
    const TOPIC: &str = "api/v1/data/kbars";

    #[cfg(feature = "channel")]
    fn assert_late_reply_log(log: &str, corr_id: &str) {
        assert!(log.contains("WARN"), "log: {log}");
        assert!(
            log.contains(
                "late reply discarded, requester already gave up (timed out or cancelled)"
            ),
            "log: {log}"
        );
        assert!(log.contains(&format!("corr_id={corr_id}")), "log: {log}");
        assert!(log.contains(&format!("topic={TOPIC}")), "log: {log}");
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    fn insert_async(client: &SolClient, corr_id: &str, id: u64) -> AsyncReceiver<SolMsg> {
        let (s, r) = bounded_async(1);
        client.inner().async_request_reply_map.insert(
            corr_id.to_string(),
            PendingAsyncRequest {
                id,
                topic: TOPIC.to_string(),
                sender: s,
            },
        );
        r
    }

    #[cfg(feature = "channel")]
    fn insert_sync(client: &SolClient, corr_id: &str, id: u64) -> Receiver<SolMsg> {
        let (s, r) = bounded(1);
        client.inner().request_reply_map.insert(
            corr_id.to_string(),
            PendingSyncRequest {
                id,
                topic: TOPIC.to_string(),
                sender: s,
            },
        );
        r
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_async_reply_delivered() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let r = insert_async(&client, "corr-ok", 1);
        let outcome = client
            .inner()
            .dispatch_reply("corr-ok", SolMsg::new().unwrap());
        assert_eq!(outcome, ReplyOutcome::Delivered);
        assert!(r.try_recv().unwrap().is_some());
        assert!(client.inner().async_request_reply_map.is_empty());
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_sync_reply_delivered() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let r = insert_sync(&client, "corr-ok-sync", 1);
        let outcome = client
            .inner()
            .dispatch_reply("corr-ok-sync", SolMsg::new().unwrap());
        assert_eq!(outcome, ReplyOutcome::Delivered);
        assert!(r.try_recv().unwrap().is_some());
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_async_late_reply_after_receiver_dropped() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let r = insert_async(&client, "corr-late", 1);
        // Requester gave up and dropped its receiver without cancelling.
        drop(r);
        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-late", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::LateReply);
        assert_late_reply_log(&log, "corr-late");
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_sync_late_reply_after_receiver_dropped() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let r = insert_sync(&client, "corr-late-sync", 1);
        drop(r);
        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-late-sync", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::LateReply);
        assert_late_reply_log(&log, "corr-late-sync");
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_reply_channel_full() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let _r = insert_async(&client, "corr-full", 1);
        let sender = client
            .inner()
            .async_request_reply_map
            .get("corr-full")
            .unwrap()
            .sender
            .clone();
        assert!(sender.try_send(SolMsg::new().unwrap()).unwrap());
        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-full", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::ChannelFull);
        assert!(log.contains("ERROR"), "log: {log}");
        assert!(log.contains("corr_id=corr-full"), "log: {log}");
        assert!(log.contains(&format!("topic={TOPIC}")), "log: {log}");
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_reply_after_cancel_logs_late_reply_with_topic() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let r = insert_async(&client, "corr-timeout", 1);
        // Caller's own timeout fires: it cancels and drops the receiver.
        assert!(client.cancel_request("corr-timeout"));
        drop(r);
        assert!(client.inner().async_request_reply_map.is_empty());
        assert_eq!(client.inner().abandoned_requests.len(), 1);

        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-timeout", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::LateReply);
        assert_late_reply_log(&log, "corr-timeout");
        // The abandoned record is used once and removed.
        assert_eq!(client.inner().abandoned_requests.len(), 0);
        assert!(!client.cancel_request("corr-timeout"));
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_sync_reply_after_cancel_logs_late_reply_with_topic() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let _r = insert_sync(&client, "corr-timeout-sync", 1);
        assert!(client.cancel_request("corr-timeout-sync"));
        assert!(client.inner().request_reply_map.is_empty());
        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-timeout-sync", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::LateReply);
        assert_late_reply_log(&log, "corr-timeout-sync");
        assert_eq!(client.inner().abandoned_requests.len(), 0);
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_guard_drop_abandons_then_late_reply_warns() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let _r = insert_async(&client, "corr-guard", 7);
        drop(PendingAsyncGuard {
            inner: client.inner(),
            corr_id: "corr-guard".to_string(),
            id: 7,
        });
        assert!(client.inner().async_request_reply_map.is_empty());
        assert_eq!(client.inner().abandoned_requests.len(), 1);
        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-guard", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::LateReply);
        assert_late_reply_log(&log, "corr-guard");
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_guard_does_not_remove_request_reusing_corr_id() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        // Request A is answered.
        let ra = insert_async(&client, "corr-reuse", 1);
        let guard_a = PendingAsyncGuard {
            inner: client.inner(),
            corr_id: "corr-reuse".to_string(),
            id: 1,
        };
        assert_eq!(
            client
                .inner()
                .dispatch_reply("corr-reuse", SolMsg::new().unwrap()),
            ReplyOutcome::Delivered
        );
        assert!(ra.try_recv().unwrap().is_some());
        // Request B reuses the corr id before A's future resumes.
        let rb = insert_async(&client, "corr-reuse", 2);
        drop(guard_a);
        // B is still pending and A was not recorded as abandoned.
        assert_eq!(
            client
                .inner()
                .async_request_reply_map
                .get("corr-reuse")
                .map(|p| p.id),
            Some(2)
        );
        assert_eq!(client.inner().abandoned_requests.len(), 0);
        assert_eq!(
            client
                .inner()
                .dispatch_reply("corr-reuse", SolMsg::new().unwrap()),
            ReplyOutcome::Delivered
        );
        assert!(rb.try_recv().unwrap().is_some());
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_guard_after_completed_request_records_nothing() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let _r = insert_async(&client, "corr-done", 3);
        client
            .inner()
            .dispatch_reply("corr-done", SolMsg::new().unwrap());
        assert!(!client.inner().abandon_async_if("corr-done", 3));
        assert_eq!(client.inner().abandoned_requests.len(), 0);
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_unmatched_reply_is_not_late_reply() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let (outcome, log) = capture_logs(|| {
            client
                .inner()
                .dispatch_reply("corr-unknown", SolMsg::new().unwrap())
        });
        assert_eq!(outcome, ReplyOutcome::Unmatched);
        assert!(log.is_empty(), "log: {log}");
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_abandoned_requests_bounded_by_age_and_count() {
        let rec = AbandonedRequests::default();
        let t0 = rec.lock().now;
        rec.lock().record("old".into(), TOPIC.into());
        {
            let mut q = rec.lock();
            q.now = t0 + ABANDONED_REQUEST_TTL + Duration::from_secs(1);
            assert_eq!(q.take("old"), None);
        }
        assert_eq!(rec.len(), 0);

        {
            let mut q = rec.lock();
            for i in 0..ABANDONED_REQUEST_CAP + 1 {
                q.record(format!("c{i}"), TOPIC.into());
            }
        }
        assert_eq!(rec.len(), ABANDONED_REQUEST_CAP);
        assert_eq!(rec.lock().take("c0"), None);
        assert_eq!(rec.lock().take("c1"), Some(TOPIC.to_string()));
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_abandoned_queue_stays_time_ordered_under_concurrency() {
        let rec = Arc::new(AbandonedRequests::default());
        let handles: Vec<_> = (0..8)
            .map(|t| {
                let rec = rec.clone();
                std::thread::spawn(move || {
                    for i in 0..100 {
                        rec.lock().record(format!("t{t}-{i}"), TOPIC.into());
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let q = rec.lock();
        assert_eq!(q.entries.len(), 800);
        assert!(q
            .entries
            .iter()
            .zip(q.entries.iter().skip(1))
            .all(|(a, b)| a.at <= b.at));
    }

    /// A reply that races a cancel must not fall through to DEBUG: the cancel
    /// side holds the abandoned lock from removing the pending entry until it
    /// has recorded it, and the callback checks the record under that lock.
    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_reply_racing_cancel_sees_abandoned_record() {
        let client = Arc::new(SolClient::new(SolClientLogLevel::Notice).unwrap());
        let _r = insert_async(&client, "corr-race", 1);

        // Cancel side, stopped between removing pending and recording.
        let mut abandoned = client.inner().abandoned_requests.lock();
        let (k, p) = client
            .inner()
            .async_request_reply_map
            .remove("corr-race")
            .unwrap();

        // The reply arrives now, on the callback thread.
        let (started_tx, started_rx) = std::sync::mpsc::channel();
        let c = client.clone();
        let reply = std::thread::spawn(move || {
            started_tx.send(()).unwrap();
            c.inner()
                .dispatch_reply("corr-race", SolMsg::new().unwrap())
        });
        started_rx.recv().unwrap();
        // Give the callback time to miss pending and block on the lock.
        std::thread::sleep(Duration::from_millis(50));
        assert!(!reply.is_finished());

        abandoned.record(k, p.topic);
        drop(abandoned);
        assert_eq!(reply.join().unwrap(), ReplyOutcome::LateReply);
        assert_eq!(client.inner().abandoned_requests.len(), 0);
    }

    /// Dropping the pending sender wakes a receiver waiting on it. If that
    /// wake-up re-enters cancel_request, it must not deadlock on the
    /// abandoned lock.
    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_sender_dropped_outside_abandoned_lock() {
        use std::future::Future;
        use std::task::{Context, Poll, Wake, Waker};

        struct ReentrantWaker {
            client: Arc<SolClient>,
            woke: std::sync::atomic::AtomicBool,
        }
        impl Wake for ReentrantWaker {
            fn wake(self: Arc<Self>) {
                self.wake_by_ref();
            }
            fn wake_by_ref(self: &Arc<Self>) {
                // Would self-deadlock if the caller still held the lock.
                self.client.cancel_request("corr-other");
                self.woke.store(true, Ordering::SeqCst);
            }
        }

        let client = Arc::new(SolClient::new(SolClientLogLevel::Notice).unwrap());
        for use_guard in [false, true] {
            let r = insert_async(&client, "corr-wake", 5);
            let waker_state = Arc::new(ReentrantWaker {
                client: client.clone(),
                woke: std::sync::atomic::AtomicBool::new(false),
            });
            let waker = Waker::from(waker_state.clone());
            let mut cx = Context::from_waker(&waker);
            let mut fut = Box::pin(r.recv());
            assert!(fut.as_mut().poll(&mut cx).is_pending());

            let (tx, rx) = std::sync::mpsc::channel();
            let c = client.clone();
            std::thread::spawn(move || {
                if use_guard {
                    drop(PendingAsyncGuard {
                        inner: c.inner(),
                        corr_id: "corr-wake".to_string(),
                        id: 5,
                    });
                } else {
                    c.cancel_request("corr-wake");
                }
                tx.send(()).unwrap();
            });
            rx.recv_timeout(Duration::from_secs(5))
                .expect("cancel deadlocked while dropping the sender");
            assert!(waker_state.woke.load(Ordering::SeqCst));
            assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Ready(Err(_))));
            assert!(client
                .inner()
                .abandoned_requests
                .lock()
                .take("corr-wake")
                .is_some());
        }
    }

    #[cfg(all(feature = "channel", feature = "tokio"))]
    #[test]
    fn test_failed_async_send_removes_pending_entry() {
        // No session: sendRequest cannot return InProgress.
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let mut msg = SolMsg::new().unwrap();
        msg.set_topic(TOPIC);
        msg.set_correlation_id("corr-fail");
        assert!(client.send_request_async_receiver(&msg).is_err());
        assert!(client.inner().async_request_reply_map.is_empty());
    }

    #[cfg(feature = "channel")]
    #[test]
    fn test_failed_sync_send_removes_pending_entry() {
        let client = SolClient::new(SolClientLogLevel::Notice).unwrap();
        let mut msg = SolMsg::new().unwrap();
        msg.set_topic(TOPIC);
        msg.set_correlation_id("corr-fail-sync");
        assert!(client.send_request(&msg, 0).is_err());
        assert!(client.inner().request_reply_map.is_empty());
    }
}
