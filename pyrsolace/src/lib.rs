use std::sync::Arc;
use std::borrow::Cow;
use std::time::Duration;
use std::thread::JoinHandle;

use pyo3::basic::CompareOp;
// use chrono::DateTime;
use pyo3::prelude::*;
// use pyo3::types::PyFunction;
use pyo3::types::PyTuple;
use pyo3::exceptions::PyException;

use rsolace::solclient::{SessionProps, SolClient, SolClientError};
use rsolace::solevent::SolEvent;
use rsolace::solmsg::{Destination, SolMsg};
use rsolace::solcache::CacheSessionProps;
use rsolace::types::{SolClientDeliveryMode, SolClientCacheRequestFlags, SolClientDestType, SolClientReturnCode, SolClientSessionEvent, SolClientSubscribeFlags, SolClientCacheStatus};

use crossbeam::atomic::AtomicCell;
use crossbeam_channel::{Receiver, RecvError};
// use rayon::{ThreadPool, ThreadPoolBuilder};
// use once_cell::sync::Lazy;


// pub static THREAD_POOL: Lazy<ThreadPool> = Lazy::new(|| {
//     let thread_name = std::env::var("RSOLACE_THREAD_NAME").unwrap_or_else(|_| "rsolace".to_string());
//     ThreadPoolBuilder::new().num_threads(
//         std::env::var("RSOLACE_MAX_THREADS").map(|s| s.parse::<usize>().expect("interger")).unwrap_or_else(|_| {
//             std::thread::available_parallelism()
//             .unwrap_or(std::num::NonZeroUsize::new(1).unwrap()).get()
//         })
//     )
//     .thread_name(move |i| format!("{}-{}", thread_name, i))
//     .build().expect("could not build thread pool")
// });

struct ReceiverError(RecvError);

impl From<ReceiverError> for PyErr {
    fn from(error: ReceiverError) -> Self {
        PyException::new_err(format!("ReceiverError: {:?}", error.0))
    }
}

impl From<RecvError> for ReceiverError {
    fn from(error: RecvError) -> Self {
        ReceiverError(error)
    }
}

#[pyclass]
struct MsgReceiver(Receiver<SolMsg>);

#[pymethods]
impl MsgReceiver {
    fn __repr__(&self) -> String {
        format!("MsgReceiver({:?})", self.0)
    }

    fn recv(&self) -> PyResult<Msg> {
        self.0.recv().map_err(|e| ReceiverError(e).into()).map(Msg::new)
    }
}



struct PySolClientError(SolClientError);

impl From<PySolClientError> for PyErr {
    fn from(error: PySolClientError) -> Self {
        PyException::new_err(format!("SolClientError: {:?}", error.0))
    }
}

impl From<SolClientError> for PySolClientError {
    fn from(other: SolClientError) -> Self {
        Self(other)
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
struct CacheStatus(SolClientCacheStatus);

#[pymethods]
impl CacheStatus {
    #[classattr]
    #[allow(non_snake_case)] 
    fn Invalid() -> Self {
        CacheStatus(SolClientCacheStatus::Invalid)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Live() -> Self {
        CacheStatus(SolClientCacheStatus::Live)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Cache() -> Self {
        CacheStatus(SolClientCacheStatus::Cache)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Suspect() -> Self {
        CacheStatus(SolClientCacheStatus::Suspect)
    }

    #[getter]
    fn value(&self) -> i32 {
        self.0 as i32
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("CacheStatus.{:?}", self.0)
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }

}

#[pyclass]
#[derive(Debug, Clone, Copy)]
struct CacheRequestFlag(SolClientCacheRequestFlags);

#[pymethods]
impl CacheRequestFlag {
    #[classattr]
    #[allow(non_snake_case)] 
    fn LiveDataFlowThru() -> Self {
        CacheRequestFlag(SolClientCacheRequestFlags::LiveDataFlowThru)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn LiveDataFulfill() -> Self {
        CacheRequestFlag(SolClientCacheRequestFlags::LiveDataFulfill)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn LiveDataQueue() -> Self {
        CacheRequestFlag(SolClientCacheRequestFlags::LiveDataQueue)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn NoSubscribe() -> Self {
        CacheRequestFlag(SolClientCacheRequestFlags::NoSubscribe)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn NowaitReply() -> Self {
        CacheRequestFlag(SolClientCacheRequestFlags::NowaitReply)
    }

    #[getter]
    fn value(&self) -> u32 {
        self.0 as u32
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("CacheRequestFlag.{:?}", self.0)
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }
}





#[pyclass]
#[derive(Debug, Clone, Copy)]
struct SubscribeFlag(SolClientSubscribeFlags);


#[pymethods]
impl SubscribeFlag {
    #[classattr]
    #[allow(non_snake_case)] 
    fn WaitForConfirm() -> Self {
        SubscribeFlag(SolClientSubscribeFlags::WaitForConfirm)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn RequestConfirm() -> Self {
        SubscribeFlag(SolClientSubscribeFlags::RequestConfirm)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn LocalDispatchOnly() -> Self {
        SubscribeFlag(SolClientSubscribeFlags::LocalDispatchOnly)
    }

    #[getter]
    fn value(&self) -> u32 {
        self.0 as u32
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("SubscribeFlag.{:?}", self.0)
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }
}


#[pyclass]
#[derive(Debug, Clone, Copy)]
struct ReturnCode(SolClientReturnCode);

#[pymethods]
impl ReturnCode {

    #[classattr]
    #[allow(non_snake_case)] 
    fn Ok() -> Self {
        ReturnCode(SolClientReturnCode::Ok)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn WouldBlock() -> Self {
        ReturnCode(SolClientReturnCode::WouldBlock)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn InProgress() -> Self {
        ReturnCode(SolClientReturnCode::InProgress)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn NotReady() -> Self {
        ReturnCode(SolClientReturnCode::NotReady)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn Eos() -> Self {
        ReturnCode(SolClientReturnCode::Eos)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn NotFound() -> Self {
        ReturnCode(SolClientReturnCode::NotFound)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn NoEvent() -> Self {
        ReturnCode(SolClientReturnCode::NoEvent)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn InComplete() -> Self {
        ReturnCode(SolClientReturnCode::InComplete)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn Rollback() -> Self {
        ReturnCode(SolClientReturnCode::Rollback)
    }


    #[classattr]
    #[allow(non_snake_case)] 
    fn Fail() -> Self {
        ReturnCode(SolClientReturnCode::Fail)
    }

    #[getter]
    fn value(&self) -> u32 {
        self.0 as u32
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("ReturnCode.{:?}", self.0)
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }

}


#[pyclass]
#[derive(Debug, Clone, Copy)]
struct LogLevel(tracing::Level);

#[pymethods]
impl LogLevel {
 
    #[classattr]
    #[allow(non_snake_case)] 
    fn Debug() -> Self {
        LogLevel(tracing::Level::DEBUG)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Info() -> Self {
        LogLevel(tracing::Level::INFO)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Warn() -> Self {
        LogLevel(tracing::Level::WARN)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Error() -> Self {
        LogLevel(tracing::Level::ERROR)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0.as_str())
    }

    fn __repr__(&self) -> String {
        format!("LogLevel.{}", self.0.as_str())
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }
}



#[pyclass]
#[derive(Debug, Clone)]
struct Dest(Destination);

#[pymethods]
impl Dest {
    #[new]
    fn __new__(dest_type: DestType, dest_name: &str) -> Self {
        Dest(Destination::new(dest_type.0, dest_name))
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }


    #[getter]
    fn get_dest_type(&self) -> DestType {
        DestType(self.0.dest_type)
    }

    #[getter]
    fn get_dest_name(&self) -> String  {
        self.0.dest.clone()
    }

    #[setter]
    fn set_dest_name(&mut self, dest_name: &str) {
        self.0.dest = dest_name.to_string();
    }

    #[setter]
    fn set_dest_type(&mut self, dest_type: DestType) {
        self.0.dest_type = dest_type.0;
    }
}


#[pyclass]
#[derive(Debug, Clone, Copy)]
struct DeliveryMode(SolClientDeliveryMode);

#[pymethods]
impl DeliveryMode {
    #[classattr]
    #[allow(non_snake_case)] 
    fn Direct() -> Self {
        DeliveryMode(SolClientDeliveryMode::Direct)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Persistent() -> Self {
        DeliveryMode(SolClientDeliveryMode::Persistent)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn NonPersistent() -> Self {
        DeliveryMode(SolClientDeliveryMode::NonPersistent)
    }

    #[getter]
    fn value(&self) -> u32 {
        self.0 as u32
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("DeliveryMode.{:?}", self.0)
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }

}

#[pyclass]
#[derive(Debug, Clone, Copy)]
struct DestType(SolClientDestType);

#[pymethods]
impl DestType {
    #[getter]
    fn value(&self) -> i32 {
        self.0 as i32
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("DestType.{:?}", self.0)
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Null() -> Self {
        DestType(SolClientDestType::Null)
    }

    #[classattr]    
    #[allow(non_snake_case)] 
    fn Topic() -> Self {
        DestType(SolClientDestType::Topic)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn Queue() -> Self {
        DestType(SolClientDestType::Queue)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn TopicTemp() -> Self {
        DestType(SolClientDestType::TopicTemp)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn QueueTemp() -> Self {
        DestType(SolClientDestType::QueueTemp)
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }
}



#[pyclass]
#[derive(Debug, Clone, Copy)]
struct SessionEvent(SolClientSessionEvent);

#[pymethods]
impl SessionEvent {
    #[classattr]
    #[allow(non_snake_case)] 
    fn UpNotice() -> Self {
        SessionEvent(SolClientSessionEvent::UpNotice)
    }

    #[classattr]
    #[allow(non_snake_case)] 
    fn DownError() -> Self {
        SessionEvent(SolClientSessionEvent::DownError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ConnectFailedError() -> Self {
        SessionEvent(SolClientSessionEvent::ConnectFailedError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn RejectedMsgError() -> Self {
        SessionEvent(SolClientSessionEvent::RejectedMsgError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn SubscriptionError() -> Self {
        SessionEvent(SolClientSessionEvent::SubscriptionError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn RxMsgTooBigError() -> Self {
        SessionEvent(SolClientSessionEvent::RxMsgTooBigError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn Acknowledgement() -> Self {
        SessionEvent(SolClientSessionEvent::Acknowledgement)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn AssuredPublishingUp() -> Self {
        SessionEvent(SolClientSessionEvent::AssuredPublishingUp)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn AssuredConnectFailed() -> Self {
        SessionEvent(SolClientSessionEvent::AssuredConnectFailed)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn TeUnsubscribeError() -> Self {
        SessionEvent(SolClientSessionEvent::TeUnsubscribeError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn TeUnsubscribeOk() -> Self {
        SessionEvent(SolClientSessionEvent::TeUnsubscribeOk)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn CanSend() -> Self {
        SessionEvent(SolClientSessionEvent::CanSend)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ReconnectingNotice() -> Self {
        SessionEvent(SolClientSessionEvent::ReconnectingNotice)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ReconnectedNotice() -> Self {
        SessionEvent(SolClientSessionEvent::ReconnectedNotice)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ProvisionError() -> Self {
        SessionEvent(SolClientSessionEvent::ProvisionError)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ProvisionOk() -> Self {
        SessionEvent(SolClientSessionEvent::ProvisionOk)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn SubscriptionOk() -> Self {
        SessionEvent(SolClientSessionEvent::SubscriptionOk)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn VirtualRouterNameChanged() -> Self {
        SessionEvent(SolClientSessionEvent::VirtualRouterNameChanged)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ModifyPropOk() -> Self {
        SessionEvent(SolClientSessionEvent::ModifyPropOk)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn ModifyPropFail() -> Self {
        SessionEvent(SolClientSessionEvent::ModifyPropFail)
    }

    #[classattr]
    #[allow(non_snake_case)]
    fn RepublishUnackedMessages() -> Self {
        SessionEvent(SolClientSessionEvent::RepublishUnackedMessages)
    }

    fn __repr__(&self) -> String {
        format!("PySolClientSessionEvent.{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[getter]
    fn value(&self) -> u32 {
        self.0 as u32
    }

    #[getter]
    fn name(&self) -> String {
        self.__str__()
    }
    
    fn __richcmp__(&self, other: &Self, op: CompareOp) -> bool {
        match op {
            CompareOp::Lt => self.0 < other.0,
            CompareOp::Le => self.0 <= other.0,
            CompareOp::Eq => self.0 == other.0,
            CompareOp::Ne => self.0 != other.0,
            CompareOp::Ge => self.0 >= other.0,
            CompareOp::Gt => self.0 > other.0,
        }
    }

}

#[pyclass]
struct Event(SolEvent);

impl Event {
    pub fn new(event: SolEvent) -> Self {
        Event(event)
    }
}

#[pymethods]
impl Event {
    // #[new]
    // fn __new__() -> Self {
    //     let event = SolEvent::new(SolClientSessionEvent::UpNotice, 0, "test".into());
    //     Event(event)
    // }

    fn get_session_event_string(&self) -> String {
        self.0.get_session_event_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    #[getter]
    fn info(&self) -> String {
        self.0.info.clone()
    }

    #[getter]
    fn response_code(&self) -> u32 {
        self.0.response_code
    }

    #[getter]
    fn session_event(&self) -> SessionEvent {
        SessionEvent(self.0.session_event)
    }

}

#[pyclass]
struct Msg(SolMsg);

impl Msg {
    pub fn new(msg: SolMsg) -> Self {
        Msg(msg)
    }
}


#[pymethods]
impl Msg {
    #[new]
    fn __new__(
        topic: Option<&str>,
        data: Option<&[u8]>,
        corr_id: Option<&str>,
        reply_topic: Option<&str>,
        is_reply: Option<bool>,
        eligible: Option<bool>,
        cos: Option<u32>,
        is_delivery_to_one: Option<bool>,
    ) -> PyResult<Self> {
        let mut msg = SolMsg::new().unwrap();
        if let Some(topic) = topic {
            msg.set_topic(topic);
        }
        if let Some(data) = data {
            msg.set_binary_attachment(data);
        }
        if let Some(reply_topic) = reply_topic {
            msg.set_reply_topic(reply_topic);
        }
        if let Some(is_reply) = is_reply {
            msg.set_as_reply(is_reply);
        }
        if let Some(eligible) = eligible {
            msg.set_eliding_eligible(eligible);
        }
        if let Some(cos) = cos {
            msg.set_class_of_service(cos);
        }
        if let Some(is_delivery_to_one) = is_delivery_to_one {
            msg.set_delivery_to_one(is_delivery_to_one);
        }
        if let Some(corr_id) = corr_id {
            msg.set_correlation_id(corr_id);
        }
        Ok(Msg(msg))
    }

    #[setter(delivery_mode)]
    fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
        self.0.set_delivery_mode(delivery_mode.0);
    }

    #[getter(delivery_mode)]
    fn get_delivery_mode(&mut self) -> Option<DeliveryMode> {
        self.0.get_delivery_mode().ok().map(|delivery_mode| DeliveryMode(delivery_mode))
    }

    #[setter(dest)]
    fn set_dest(&mut self, dest: Dest) {
        self.0.set_destination(&dest.0);
    }

    #[getter(dest)]
    fn get_dest(&self) -> Option<Dest> {
        self.0.get_destination().ok().map(|dest| Dest(dest))
    }

    #[setter(reply_to)]
    fn set_reply_to(&mut self, reply_to: Dest) {
        self.0.set_reply_to(&reply_to.0);
    }

    #[getter(reply_to)]
    fn get_reply_to(&self) -> Option<Dest> {
        self.0.get_reply_to().ok().map(|dest| Dest(dest))
    }

    #[setter(is_reply)]
    fn set_as_reply(&mut self, is_reply: bool) {
        self.0.set_as_reply(is_reply);
    }

    #[getter(is_reply)]
    fn is_reply(&self) -> bool {
        self.0.is_reply()
    }

    #[setter(eligible)]
    fn set_eligible(&mut self, eligible: bool) {
        self.0.set_eliding_eligible(eligible);
    }

    #[getter(eligible)]
    fn is_eligible(&self) -> bool {
        self.0.is_eliding_eligible()
    }

    #[getter]
    fn is_p2p(&self) -> bool {
        self.0.is_p2p()
    }

    #[setter(corr_id)]
    fn set_corr_id(&mut self, corr_id: &str) {
        self.0.set_correlation_id(corr_id);
    }

    #[getter(corr_id)]
    fn get_corr_id(&self) -> Option<String> {
        self.0.get_correlation_id().ok()
    }

    #[getter(cos)]
    fn get_cos(&self) -> u32 {
        self.0.get_class_of_service().unwrap()
    }

    #[setter(cos)]
    fn set_cos(&mut self, cos: u32) {
        self.0.set_class_of_service(cos);
    }

    #[getter(is_delivery_to_one)]
    fn get_delivery_to_one(&self) -> bool {
        self.0.is_delivery_to_one()
    }

    #[setter(is_delivery_to_one)]
    fn set_delivery_to_one(&mut self, delivery_to_one: bool) {
        self.0.set_delivery_to_one(delivery_to_one);
    }

    #[setter(topic)]
    fn set_topic(&mut self, topic: &str) {
        self.0.set_topic(topic);
    }

    #[getter(topic)]
    fn get_topic(&self) -> Option<String> {
        self.0.get_topic().ok()
    }

    #[setter(reply_topic)]
    fn set_reply_topic(&mut self, reply_topic: &str) {
        self.0.set_reply_topic(reply_topic);
    }

    #[getter(reply_topic)]
    fn get_reply_topic(&self) -> Option<String> {
        self.0.get_reply_topic().ok()
    }

    // not allow for abi3
    // #[getter(sender_time)]
    // fn get_sender_time(&self) -> Option<DateTime<chrono::Utc>> {
    //     self.0.get_sender_time().ok()
    // }

    #[getter(sender_timestamp)]
    fn get_sender_time(&self) -> Option<i64> {
        self.0.get_sender_ts().ok()
    }

    #[setter(sender_timestamp)]
    fn set_sender_time(&mut self, sender_time: i64) {
        self.0.set_sender_ts(sender_time);
    }

    #[getter(recv_timestamp)]
    fn get_recv_time(&self) -> Option<i64> {
        self.0.get_recv_ts().ok()
    }

    #[getter(is_cache)]
    fn is_cache(&self) -> bool {
        self.0.is_cache()
    }

    #[getter(cache_status)]
    fn get_cache_status(&self) -> CacheStatus {
        CacheStatus(self.0.get_cache_status())
    }
    

    #[getter(sender_id)]
    fn get_sender_id(&self) -> Option<String> {
        self.0.get_sender_id().ok()
    }

    #[setter(sender_id)]
    fn set_sender_id(&mut self, sender_id: &str) {
        self.0.set_sender_id(sender_id);
    }


    #[getter(seq)]
    fn get_seq(&self) -> Option<i64> {
        self.0.get_seq().ok()
    }

    #[setter(seq)]
    fn set_seq(&mut self, seq: u64) {
        self.0.set_seq(seq);
    }

    #[getter(msg_type)]
    fn get_msg_type(&self) -> Option<Cow<str>> {
        self.0.get_msg_type().ok()
    }

    #[setter(msg_type)]
    fn set_msg_type(&mut self, msg_type: &str) {
        self.0.set_msg_type(msg_type);
    }

    #[getter(cache_request_id)]
    fn get_cache_request_id(&self) -> Option<u64> {
        self.0.get_cache_request_id().ok()
    }

    #[getter(is_discard_indication)]
    fn get_is_discard_indication(&self) -> bool {
        self.0.is_discard_indication()
    }

    
    fn get_user_prop(&self, key: &str) -> String {
        self.0.get_user_prop(key).unwrap_or("".into())
    }

    #[pyo3(signature = (key, value, map_size=10))]
    fn set_user_prop(&mut self, key: &str, value: &str, map_size: u32) {
        self.0.set_user_prop(key, value, map_size);
    }

    #[setter(data)]
    fn set_data(&mut self, data: &[u8]) {
        self.0.set_binary_attachment(data);
    }

    #[getter(data)]
    fn get_data(&self) -> Cow<[u8]> {
        self.0.get_binary_attachment().unwrap_or(Cow::Borrowed(&[]))
    }

    fn dump(&self) -> Cow<str> {
        // self.0.dump(true);
        self.0.dump(false).unwrap_or("None".into())
    }

    fn __repr__(&self) -> String {
        format!("SolMsg:\n{}", self.dump())
    }
}


#[pyclass(name = "Client")]
struct Client {
    solclient: SolClient,
    is_connected: bool,
    // event_callback: Option<Py<PyFunction>>, // callable
    // msg_callback: Option<Py<PyFunction>>, // callable
    event_callback: Option<Py<PyAny>>, // callable
    msg_callback: Option<Py<PyAny>>, // callable
    request_callback: Option<Py<PyAny>>, // callable
    p2p_callback: Option<Py<PyAny>>, // callable
    th_event_join: Option<JoinHandle<()>>,
    th_msg_join: Option<JoinHandle<()>>,
    th_request_join: Option<JoinHandle<()>>,
    th_p2p_join: Option<JoinHandle<()>>,
    msg_break: Arc<AtomicCell<bool>>,
    event_break: Arc<AtomicCell<bool>>,
    request_break: Arc<AtomicCell<bool>>,
    p2p_break: Arc<AtomicCell<bool>>,
}

#[pyfunction]
#[pyo3(signature = (
    level=LogLevel::Info(), 
    display_line_number=false, 
    display_thread_names=false, 
    display_thread_ids=false, 
    display_filename=false)
)]
fn init_tracing_logger(
    level: LogLevel, 
    display_line_number: bool, 
    display_thread_names: bool, 
    display_thread_ids: bool, 
    display_filename: bool
) {
    tracing_subscriber::fmt()
    .with_max_level(level.0)
    .with_line_number(display_line_number)
    .with_thread_names(display_thread_names)
    .with_thread_ids(display_thread_ids)
    .with_file(display_filename)
    .init();
}

#[pymethods]
impl Client {
    #[new]
    fn __new__() -> Self {
        let solclient = SolClient::default();
        let msg_recv = solclient.get_msg_receiver();
        let msg_break = Arc::new(AtomicCell::new(false));
        let msg_break_clone = msg_break.clone();
        let th_msg_join = std::thread::spawn(move || loop {
            match msg_recv.recv_timeout(Duration::from_millis(1000)) {
                Ok(msg) => {
                    tracing::info!("{:?}", msg);
                }
                Err(_) => {
                    if msg_break_clone.load() {
                        tracing::debug!("msg_loop_break");
                        drop(msg_recv);
                        break;
                    }
                }
            }
        });
        let event_recv = solclient.get_event_receiver();
        let event_break = Arc::new(AtomicCell::new(false));
        let event_break_clone = event_break.clone();
        let th_event_join = std::thread::spawn(move || loop {
            match event_recv.recv_timeout(Duration::from_millis(1000)) {
                Ok(event) => {
                    tracing::info!("{:?}", event);
                }
                Err(_) => {
                    if event_break_clone.load() {
                        tracing::debug!("event_loop_break");
                        drop(event_recv);
                        break;
                    }
                }
            }
        });
        Client {
            solclient: solclient,
            is_connected: false,
            event_callback: None,
            msg_callback: None,
            request_callback: None,
            p2p_callback: None,
            th_event_join: Some(th_event_join),
            th_msg_join: Some(th_msg_join),
            th_request_join: None,
            th_p2p_join: None,
            msg_break: msg_break,
            event_break: event_break,
            request_break: Arc::new(AtomicCell::new(false)),
            p2p_break: Arc::new(AtomicCell::new(false)),
        }
    }

    #[pyo3(signature = (msg_callback))]
    fn set_msg_callback(&mut self, msg_callback: &PyAny){
        // self.msg_callback = msg_callback.downcast::<PyFunction>().ok().map(|f| f.into());
        self.msg_break.store(true);
        self.msg_callback = Some(msg_callback.into());
        if let Some(join_handle) = self.th_msg_join.take() {
            tracing::debug!("msg_join_handle");
            join_handle.join().unwrap();
            tracing::debug!("msg_join_handle done");    
        }
        self.msg_break.store(false);
        tracing::debug!("set msg_break to false");
        match &self.th_msg_join {
            Some(_) => {},
            None => {
                let msg_recv = self.solclient.get_msg_receiver();
                let msg_callback = self.msg_callback.as_ref().cloned().unwrap();
                let msg_break = self.msg_break.clone();
                let th_msg_join = std::thread::spawn(move || loop {
                    // tracing::debug!("msg_cb {:?}", msg_callback);
                    match msg_recv.recv_timeout(Duration::from_millis(500)) {
                        Ok(msg) => {
                            let py_msg = Msg::new(msg);
                            Python::with_gil(|py| {
                                let args = PyTuple::new(py, &[py_msg.into_py(py)]); 
                                let _res = msg_callback.call1(py, args);    
                            })
                        }
                        Err(_e) => {
                            if msg_break.load() {
                                tracing::debug!("msg_break");
                                drop(msg_recv);
                                break;
                            }
                        }
                    }
                });
                self.th_msg_join = Some(th_msg_join);
            }
        }
    }

    #[pyo3(signature = (event_callback))]
    fn set_event_callback(&mut self, event_callback: &PyAny){
        // self.event_callback = event_callback.downcast::<PyFunction>().ok().map(|f| f.into());
        self.event_break.store(true);
        self.event_callback = Some(event_callback.into());
        if let Some(join_handle) = self.th_event_join.take() {
            tracing::debug!("event_join_handle");
            join_handle.join().unwrap();
            tracing::debug!("event_join_handle done");    
        }
        self.event_break.store(false);
        tracing::debug!("set event_break to false");
        match &self.th_event_join {
            Some(_) => {},
            None => {
                let event_recv = self.solclient.get_event_receiver();
                let event_callback = self.event_callback.as_ref().cloned().unwrap();
                let event_break = self.event_break.clone();
                let th_event_join = std::thread::spawn(move || loop {
                    match event_recv.recv_timeout(Duration::from_millis(500)) {
                        Ok(event) => {        
                            let py_event = Event::new(event);
                            Python::with_gil(|py| {
                                let args = PyTuple::new(py, &[py_event.into_py(py)]); 
                                let _res = event_callback.call1(py, args);
                            })
                        }
                        Err(_e) => {
                            if event_break.load() {
                                tracing::debug!("event_break");
                                drop(event_recv);
                                break;
                            }
                        }
                    }
                });
                self.th_event_join = Some(th_event_join);
            }
        }
    }

    #[pyo3(signature = (request_callback))]
    fn set_request_callback(&mut self, request_callback: &PyAny){
        self.request_break.store(true);
        self.request_callback = Some(request_callback.into());
        if let Some(join_handle) = self.th_request_join.take() {
            tracing::debug!("request_join_handle");
            join_handle.join().unwrap();
            tracing::debug!("request_join_handle done");    
        }
        self.request_break.store(false);
        match &self.th_request_join {
            Some(_) => {},
            None => {
                let request_recv = self.solclient.get_request_receiver();
                let request_callback = self.request_callback.as_ref().cloned().unwrap();
                let request_break = self.request_break.clone();
                let th_request_join = std::thread::spawn(move || loop {
                    match request_recv.recv_timeout(Duration::from_millis(500)) {
                        Ok(request) => {
                            let py_request = Msg::new(request);
                            Python::with_gil(|py| {
                                let args = PyTuple::new(py, &[py_request.into_py(py)]); 
                                let _res = request_callback.call1(py, args);
                            })   
                        }
                        Err(_e) => {
                            if request_break.load() {
                                tracing::debug!("request_break");
                                drop(request_recv);
                                break;
                            }
                        }
                    }
                });
                self.th_request_join = Some(th_request_join);
            }
        }
    }

    #[pyo3(signature = (p2p_callback))]
    fn set_p2p_callback(&mut self, p2p_callback: &PyAny){
        self.p2p_break.store(true);
        self.p2p_callback = Some(p2p_callback.into());
        if let Some(join_handle) = self.th_p2p_join.take() {
            tracing::debug!("p2p_join_handle");
            join_handle.join().unwrap();
            tracing::debug!("p2p_join_handle done");    
        }
        self.p2p_break.store(false);
        tracing::debug!("set p2p_break to false");
        match &self.th_p2p_join {
            Some(_) => {},
            None => {
                let p2p_recv = self.solclient.get_p2p_receiver();
                let p2p_callback = self.p2p_callback.as_ref().cloned().unwrap();
                let p2p_break = self.p2p_break.clone();
                let th_p2p_join = std::thread::spawn(move || loop {
                    match p2p_recv.recv_timeout(Duration::from_millis(500)) {
                        Ok(p2p) => {
                            let py_p2p = Msg::new(p2p);
                            Python::with_gil(|py| {
                                let args = PyTuple::new(py, &[py_p2p.into_py(py)]); 
                                let _res = p2p_callback.call1(py, args);
                            })
                        }
                        Err(_e) => {
                            if p2p_break.load() {
                                tracing::debug!("p2p_break");
                                drop(p2p_recv);
                                break;
                            }
                        }
                    }
                });
                self.th_p2p_join = Some(th_p2p_join);
            }
        }
    }
    

    #[pyo3(signature = (
        host, vpn, username, password, client_name="", connect_timeout_ms=3000, 
        reconnect_retries=10, keep_alive_ms=3000, reconnect_retry_wait=3000,
        keep_alive_limit=3, compression_level=1, connect_retries=3, 
        reapply_subscriptions=true, generate_sender_id=false, generate_sequence_number=false,
        generate_send_timestamps=false, generate_rcv_timestamps=false
    ))]
    fn connect(
        &mut self,
        host: &str,
        vpn: &str,
        username: &str,
        password: &str,
        client_name: &str,
        connect_timeout_ms: u32,
        reconnect_retries: u32,
        keep_alive_ms: u32,
        reconnect_retry_wait: u32,
        keep_alive_limit: u32,
        compression_level: u32,
        connect_retries: u32,
        reapply_subscriptions: bool,
        generate_sender_id: bool,
        generate_sequence_number: bool,
        generate_send_timestamps: bool,
        generate_rcv_timestamps: bool,
    ) -> bool {
        let props = SessionProps::default()
            .username(username)
            .password(password)
            .host(host)
            .vpn(vpn)
            .client_name(client_name)
            .compression_level(compression_level)
            .connect_timeout_ms(connect_timeout_ms)
            .connect_retries(connect_retries)
            .reconnect_retries(reconnect_retries)
            .reapply_subscriptions(reapply_subscriptions)
            .reconnect_retry_wait_ms(reconnect_retry_wait)
            .generate_rcv_timestamps(generate_rcv_timestamps)
            .generate_send_timestamps(generate_send_timestamps)
            .generate_sender_id(generate_sender_id)
            .generate_sequence_number(generate_sequence_number)
            .keep_alive_limit(keep_alive_limit)
            .keep_alive_int_ms(keep_alive_ms);
        let r = self.solclient.connect(props);
        self.is_connected = true;
        r
    }

    #[pyo3(signature = ())]
    fn disconnect(&mut self) {
        if self.is_connected {
            self.solclient.disconnect();
            self.is_connected = false;
        }
    }

    fn subscribe(&mut self, topic: &str) -> ReturnCode {
        ReturnCode(self.solclient.subscribe(topic))
    }

    fn unsubscribe(&mut self, topic: &str) -> ReturnCode {
        ReturnCode(self.solclient.unsubscribe(topic))
    }

    fn subscribe_ext(&mut self, topic: &str, flag: SubscribeFlag) -> ReturnCode {
        ReturnCode(self.solclient.subscribe_ext(topic, flag.0))
    }

    fn unsubscribe_ext(&mut self, topic: &str, flag: SubscribeFlag) -> ReturnCode {
        ReturnCode(self.solclient.unsubscribe_ext(topic, flag.0))
    }

    fn send_msg(&mut self, msg: &Msg) -> ReturnCode {
        ReturnCode(self.solclient.send_msg(&msg.0))
    }

    fn send_multiple_msg(&mut self, msgs: Vec<PyRef<Msg>>) -> ReturnCode {
        let m = msgs.iter().map(|msg| &msg.0).collect::<Vec<_>>();
        ReturnCode(self.solclient.send_multiple_msg(&m))
    }


    #[pyo3(signature = (topic, request_id, cache_name, max_msgs=0, max_age=0, request_reply_timeout=10000, flag=CacheRequestFlag(SolClientCacheRequestFlags::LiveDataFlowThru)))]
    fn send_cache_request(&self, topic: &str, request_id: u64, cache_name: &str, max_msgs: u32, max_age: u32, request_reply_timeout: u32, flag: CacheRequestFlag) -> Result<(), PySolClientError> {
        let props = CacheSessionProps::default()
            .cache_name(cache_name)
            .max_msgs(max_msgs)
            .max_age(max_age)
            .request_reply_timeout(request_reply_timeout);
        self.solclient.send_cache_request(topic, request_id, props, flag.0).map_err(PySolClientError::from)
    }

    fn send_request(&mut self, msg: &Msg, timeout: u32) -> Result<MsgReceiver, PySolClientError> {
        let receiver = self.solclient.send_request(&msg.0, timeout).map_err(PySolClientError::from)?;
        Ok(MsgReceiver(receiver))
    }

    fn send_reply(&self, rx_msg: &Msg, reply_msg: &Msg) -> ReturnCode {
        ReturnCode(self.solclient.send_reply(&rx_msg.0, &reply_msg.0))
    }

    fn modify_client_info(&mut self, app_description: Option<&str>, client_name: Option<&str>) -> ReturnCode {
        ReturnCode(self.solclient.modify_client_info(app_description, client_name))
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.disconnect();
        self.msg_break.store(true);
        self.event_break.store(true);
        self.request_break.store(true);
        self.p2p_break.store(true);
        if let Some(join_handle) = self.th_msg_join.take() { 
            join_handle.join().unwrap();
        }
        if let Some(join_handle) = self.th_event_join.take() { 
            join_handle.join().unwrap();
        }
        if let Some(join_handle) = self.th_request_join.take() { 
            join_handle.join().unwrap();
        }
        if let Some(join_handle) = self.th_p2p_join.take() { 
            join_handle.join().unwrap();
        }
        
    }
}



/// A Python module implemented in Rust.
#[pymodule]
fn pyrsolace(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Client>()?;
    m.add_class::<Event>()?;
    m.add_class::<Msg>()?;
    m.add_class::<DeliveryMode>()?;
    m.add_class::<SessionEvent>()?;
    m.add_class::<DestType>()?;
    m.add_class::<Dest>()?;
    m.add_class::<LogLevel>()?;
    m.add_class::<ReturnCode>()?;
    m.add_class::<SubscribeFlag>()?;
    m.add_class::<CacheStatus>()?;
    m.add_class::<CacheRequestFlag>()?;
    m.add_function(wrap_pyfunction!(init_tracing_logger, m)?)?;
    Ok(())
}
