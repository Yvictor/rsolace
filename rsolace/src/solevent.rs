use super::types::SolClientSessionEvent;
use snafu::prelude::{ensure, Snafu};
use snafu::ResultExt;
use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct SolEvent {
    pub session_event: SolClientSessionEvent,
    pub response_code: u32,
    pub info: String,
    /// Application-supplied correlation tag passed back by the C API in
    /// asynchronous Session-event confirmations (e.g.
    /// `SOLCLIENT_SESSION_EVENT_MODIFYPROP_OK` /
    /// `SOLCLIENT_SESSION_EVENT_MODIFYPROP_FAIL`). Stored as a `usize`
    /// because the underlying C field is `void *` and rsolace uses the
    /// pointer as an opaque numeric handle into an internal waiter map.
    /// `None` when the C API did not provide a correlation pointer for
    /// this event.
    pub correlation_tag: Option<usize>,
}

#[derive(Debug, Snafu)]
pub enum SolEventError {
    #[snafu(display("Null event pointer"))]
    NullEventPtr,
    #[snafu(display("Null info pointer"))]
    NullInfoPtr,
    #[snafu(display("Info UTF-8 error"))]
    InfoUtf8 { source: std::str::Utf8Error },
}

impl SolEvent {
    pub fn new(session_event: SolClientSessionEvent, response_code: u32, info: &str) -> SolEvent {
        SolEvent {
            session_event,
            response_code,
            info: info.to_string(),
            correlation_tag: None,
        }
    }

    /// # Safety
    /// 呼叫端需確保 `event_p` 來源生命週期有效。
    pub unsafe fn from_ptr(
        event_p: rsolace_sys::solClient_session_eventCallbackInfo_pt,
    ) -> Result<SolEvent, SolEventError> {
        ensure!(!event_p.is_null(), NullEventPtrSnafu);
        let event = *event_p;

        ensure!(!event.info_p.is_null(), NullInfoPtrSnafu);
        let info = CStr::from_ptr(event.info_p)
            .to_str()
            .context(InfoUtf8Snafu)?
            .to_owned();

        // The C API forwards back the `void *correlation_p` we passed into
        // the originating non-blocking call (e.g. modifyClientInfo). rsolace
        // uses the pointer as an opaque numeric tag, so reinterpret it as
        // a `usize`. A null pointer means "no correlation supplied".
        let correlation_tag = if event.correlation_p.is_null() {
            None
        } else {
            Some(event.correlation_p as usize)
        };

        Ok(SolEvent {
            session_event: event.sessionEvent.into(),
            response_code: event.responseCode,
            info,
            correlation_tag,
        })
    }

    pub fn get_session_event_string(&self) -> String {
        unsafe {
            let res = rsolace_sys::solClient_session_eventToString(
                self.session_event as rsolace_sys::solClient_session_event_t,
            );
            CStr::from_ptr(res).to_str().unwrap().to_owned()
        }
    }
}
