use std::borrow::Cow;
use std::thread::JoinHandle;

use chrono::DateTime;
use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyTuple};
use rsolace::{solcache::CacheSessionProps, solclient::{SessionProps, SolClient, SolClientError}, solevent::SolEvent, solmsg::Destination, types::{SolClientCacheRequestFlags, SolClientDestType, SolClientReturnCode, SolClientSessionEvent, SolClientSubscribeFlags}};

use rsolace::solmsg::SolMsg;
use rsolace::types::SolClientDeliveryMode;
use pyo3::exceptions::PyException;

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
    fn Error() -> Self {
        LogLevel(tracing::Level::ERROR)
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
}



#[pyclass]
#[derive(Debug, Clone, Copy)]
struct SessionEvent(SolClientSessionEvent);

#[pymethods]
impl SessionEvent {
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
    fn __new__() -> PyResult<Self> {
        Ok(Msg(SolMsg::new().unwrap()))
    }

    #[setter(delivery_mode)]
    fn set_delivery_mode(&mut self, delivery_mode: DeliveryMode) {
        self.0.set_delivery_mode(delivery_mode.0);
    }

    #[getter(delivery_mode)]
    fn get_delivery_mode(&mut self) -> DeliveryMode {
        DeliveryMode(self.0.get_delivery_mode().unwrap())
    }

    #[setter(dest)]
    fn set_dest(&mut self, dest: Dest) {
        self.0.set_destination(&dest.0);
    }

    #[getter(dest)]
    fn get_dest(&self) -> Dest {
        Dest(self.0.get_destination().unwrap())
    }

    #[setter(reply_to)]
    fn set_reply_to(&mut self, reply_to: Dest) {
        self.0.set_reply_to(&reply_to.0);
    }

    #[getter(reply_to)]
    fn get_reply_to(&self) -> Dest {
        Dest(self.0.get_reply_to().unwrap())
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
    fn get_corr_id(&self) -> String {
        self.0.get_correlation_id().unwrap()
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
    fn get_topic(&self) -> String {
        self.0.get_topic().unwrap()
    }

    #[setter(reply_topic)]
    fn set_reply_topic(&mut self, reply_topic: &str) {
        self.0.set_reply_topic(reply_topic);
    }

    #[getter(reply_topic)]
    fn get_reply_topic(&self) -> String {
        self.0.get_reply_topic().unwrap()
    }

    #[getter(sender_time)]
    fn get_sender_time(&self) -> DateTime<chrono::Utc> {
        self.0.get_sender_time().unwrap()
    }

    
    fn get_user_prop(&self, key: &str) -> String {
        self.0.get_user_prop(key).unwrap()
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
        // PyBytes::new(py, &self.0.get_binary_attachment().unwrap())
        self.0.get_binary_attachment().unwrap().into()
        // self.0.get_binary_attachment().unwrap().into()
    }

    fn dump(&self) -> String {
        self.0.dump(false).unwrap()
    }

    fn __repr__(&self) -> String {
        format!("SolMsg:\n{}", self.dump())
    }
}



#[pyclass(name = "Client")]
struct Client {
    solclient: SolClient,
    event_callback: Option<Py<PyFunction>>, // callable
    msg_callback: Option<Py<PyFunction>>, // callable
    th_event_join: Option<JoinHandle<()>>,
    th_msg_join: Option<JoinHandle<()>>,
}

#[pyfunction]
fn init_tracing_logger(level: LogLevel) {
    tracing_subscriber::fmt().with_max_level(level.0).init();
}

#[pymethods]
impl Client {
    #[new]
    fn __new__() -> Self {
        let solclient = SolClient::default();
        Client {
            solclient: solclient,
            event_callback: None,
            msg_callback: None,
            th_event_join: None,
            th_msg_join: None,
        }
    }

    #[pyo3(signature = (msg_callback))]
    fn set_msg_callback(&mut self, msg_callback: &PyFunction){
        // self.msg_callback = msg_callback.downcast::<PyFunction>().ok().map(|f| f.into());
        self.msg_callback = Some(msg_callback.into());
        match &self.th_msg_join {
            Some(_) => {},
            None => {
                let msg_recv = self.solclient.get_msg_receiver();
                let msg_callback = self.msg_callback.as_ref().cloned();
                let th_msg_join = std::thread::spawn(move || loop {
                    match msg_recv.recv() {
                        Ok(msg) => {
                            // tracing::info!("recv msg");
                            let py_msg = Msg::new(msg);
                            Python::with_gil(|py| {
                                let args = PyTuple::new(py, &[py_msg.into_py(py)]); 
                                if let Some(msg_cb) = &msg_callback {
                                    let _res = msg_cb.call1(py, args);
                                }
                            })
                        }
                        Err(e) => {
                            tracing::error!("recv msg error: {:?}", e);
                            break;
                        }
                    }
                });
                self.th_msg_join = Some(th_msg_join);
            }
        }
    }

    #[pyo3(signature = (event_callback))]
    fn set_event_callback(&mut self, event_callback: &PyFunction){
        // self.event_callback = event_callback.downcast::<PyFunction>().ok().map(|f| f.into());
        self.event_callback = Some(event_callback.into());
        tracing::debug!("set_event_callback: {:?}", self.event_callback);
        match &self.th_event_join {
            Some(_) => {},
            None => {
                let event_recv = self.solclient.get_event_receiver();
                let event_callback = self.event_callback.as_ref().cloned();
                let th_event_join = std::thread::spawn(move || loop {
                    match event_recv.recv() {
                        Ok(event) => {
                            tracing::info!("recv event");
                            let py_event = Event::new(event);
                            Python::with_gil(|py| {
                                // let event_info = PyString::new(py, &event.info);
                                let args = PyTuple::new(py, &[py_event.into_py(py)]); 
                                if let Some(event_cb) = &event_callback {
                                    let _res = event_cb.call1(py, args);
                                }
                            })
                        }
                        Err(e) => {
                            tracing::error!("recv event error: {:?}", e);
                            break;
                        }
                    }
                });
                self.th_event_join = Some(th_event_join);
            }
        }
    }

    #[pyo3(signature = (
        host, vpn, username, password, clientname="", connect_timeout_ms=3000, 
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
        clientname: &str,
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
            .client_name(clientname)
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
        self.solclient.connect(props)
    }

    #[pyo3(signature = ())]
    fn disconnect(&mut self) {
        self.solclient.disconnect()
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

    // fn send_multiple_msg(&mut self, msgs: &[Msg]) -> ReturnCode {
    //     ReturnCode(self.solclient.send_multiple_msg(&msgs.iter().map(|msg| &msg.0).collect::<Vec<_>>()))
    // }

    #[pyo3(signature = (topic, request_id, cache_name, max_msgs=0, max_age=0, request_reply_timeout=10000, flag=CacheRequestFlag(SolClientCacheRequestFlags::LiveDataFlowThru)))]
    fn send_cache_request(&self, topic: &str, request_id: u64, cache_name: &str, max_msgs: u32, max_age: u32, request_reply_timeout: u32, flag: CacheRequestFlag) -> Result<(), PySolClientError> {
        let props = CacheSessionProps::default()
            .cache_name(cache_name)
            .max_msgs(max_msgs)
            .max_age(max_age)
            .request_reply_timeout(request_reply_timeout);
        self.solclient.send_cache_request(topic, request_id, props, flag.0).map_err(PySolClientError::from)
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
    m.add_function(wrap_pyfunction!(init_tracing_logger, m)?)?;
    Ok(())
}
