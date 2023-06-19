use super::solevent::SolEvent;
use super::solmsg::{SolMsg, SolMsgError};
use super::types::{SolClientLogLevel, SolClientReturnCode, SolClientSubscribeFlags};
use super::utils::ConvertToCString;
use enum_primitive::FromPrimitive;
use snafu::prelude::{ensure, Snafu};
use snafu::ResultExt;
use std::ffi::{c_void, CString};
use std::option::Option;
use std::ptr::{null, null_mut};
// TODO fn pointer to struct

#[derive(Debug, Snafu, PartialEq)]
pub enum SolClientError {
    #[snafu(display("SolClient context create Error"))]
    ContextCreate,
    #[snafu(display("SolClient send request {topic} Error"))]
    SendRequest { topic: String },
    #[snafu(display("SolClient inside {}", source))]
    SolMsg { source: SolMsgError },
}

#[derive(Debug)]
pub struct SessionProps {
    username: CString,
    password: CString,
    host: CString,
    vpn: CString,
    client_name: CString,
    connect_timeout_ms: CString,
    tcp_nodelay: CString,
    keep_alive_int_ms: CString,
    keep_alive_limit: CString,
    // buffer_size: u32,
    compression_level: CString,
    generate_rcv_timestamps: CString,
    generate_send_timestamps: CString,
    generate_sender_id: CString,
    generate_sequence_number: CString,
    connect_retries: CString,
    reconnect_retries: CString,
    reconnect_retry_wait_ms: CString,
    reapply_subscriptions: CString,
}

impl SessionProps {
    pub fn to_c(&self) -> [*const i8; 37] {
        let session_props = [
            rsolace_sys::SOLCLIENT_SESSION_PROP_HOST.as_ptr() as *const i8,
            self.host.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_VPN_NAME.as_ptr() as *const i8,
            self.vpn.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_USERNAME.as_ptr() as *const i8,
            self.username.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_PASSWORD.as_ptr() as *const i8,
            self.password.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_COMPRESSION_LEVEL.as_ptr() as *const i8,
            self.compression_level.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_CLIENT_NAME.as_ptr() as *const i8,
            self.client_name.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_CONNECT_TIMEOUT_MS.as_ptr() as *const i8,
            self.connect_timeout_ms.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_TCP_NODELAY.as_ptr() as *const i8,
            self.tcp_nodelay.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_RCV_TIMESTAMPS.as_ptr() as *const i8,
            self.generate_rcv_timestamps.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SEND_TIMESTAMPS.as_ptr() as *const i8,
            self.generate_send_timestamps.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SENDER_ID.as_ptr() as *const i8,
            self.generate_sender_id.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SEQUENCE_NUMBER.as_ptr() as *const i8,
            self.generate_sequence_number.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_CONNECT_RETRIES.as_ptr() as *const i8,
            self.connect_retries.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_RECONNECT_RETRIES.as_ptr() as *const i8,
            self.reconnect_retries.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_RECONNECT_RETRY_WAIT_MS.as_ptr() as *const i8,
            self.reconnect_retry_wait_ms.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_REAPPLY_SUBSCRIPTIONS.as_ptr() as *const i8,
            self.reapply_subscriptions.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_KEEP_ALIVE_INT_MS.as_ptr() as *const i8,
            self.keep_alive_int_ms.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_KEEP_ALIVE_LIMIT.as_ptr() as *const i8,
            self.keep_alive_limit.as_ptr() as *const i8,
            null(),
        ];
        session_props
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_cstring();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_cstring();
        self
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_cstring();
        self
    }

    pub fn vpn(mut self, vpn: &str) -> Self {
        self.vpn = vpn.to_cstring();
        self
    }

    pub fn compression_level(mut self, compression_level: u32) -> Self {
        assert!(compression_level < 10);
        self.compression_level = compression_level.to_cstring();
        self
    }

    pub fn connect_timeout_ms(mut self, timeout: u32) -> Self {
        self.connect_timeout_ms = timeout.to_cstring();
        self
    }

    pub fn tcp_nodelay(mut self, enable: bool) -> Self {
        self.tcp_nodelay = enable.to_cstring();
        self
    }

    pub fn client_name(mut self, client_name: &str) -> Self {
        self.client_name = client_name.to_cstring();
        self
    }

    pub fn keep_alive_int_ms(mut self, keep_alive_int_ms: u32) -> Self {
        self.keep_alive_int_ms = keep_alive_int_ms.to_cstring();
        self
    }

    pub fn keep_alive_limit(mut self, keep_alive_limit: u32) -> Self {
        self.keep_alive_limit = keep_alive_limit.to_cstring();
        self
    }

    pub fn generate_rcv_timestamps(mut self, generate_rcv_timestamps: bool) -> Self {
        self.generate_rcv_timestamps = generate_rcv_timestamps.to_cstring();
        self
    }

    pub fn generate_send_timestamps(mut self, generate_send_timestamps: bool) -> Self {
        self.generate_send_timestamps = generate_send_timestamps.to_cstring();
        self
    }

    pub fn generate_sender_id(mut self, generate_sender_id: bool) -> Self {
        self.generate_sender_id = generate_sender_id.to_cstring();
        self
    }

    pub fn generate_sequence_number(mut self, generate_sequence_number: bool) -> Self {
        self.generate_sequence_number = generate_sequence_number.to_cstring();
        self
    }

    pub fn connect_retries(mut self, connect_retries: u32) -> Self {
        self.connect_retries = connect_retries.to_cstring();
        self
    }

    pub fn reconnect_retries(mut self, reconnect_retries: u32) -> Self {
        self.reconnect_retries = reconnect_retries.to_cstring();
        self
    }

    pub fn reconnect_retry_wait_ms(mut self, reconnect_retry_wait_ms: u32) -> Self {
        self.reconnect_retry_wait_ms = reconnect_retry_wait_ms.to_cstring();
        self
    }

    pub fn reapply_subscriptions(mut self, reapply_subscriptions: bool) -> Self {
        self.reapply_subscriptions = reapply_subscriptions.to_cstring();
        self
    }
}

impl std::default::Default for SessionProps {
    fn default() -> Self {
        Self {
            username: "".to_cstring(),
            password: "".to_cstring(),
            host: "".to_cstring(),
            vpn: "".to_cstring(),
            client_name: "".to_cstring(),
            connect_timeout_ms: 30000.to_cstring(),
            tcp_nodelay: true.to_cstring(),
            keep_alive_int_ms: 3000.to_cstring(),
            keep_alive_limit: 3.to_cstring(),
            compression_level: 0.to_cstring(),
            generate_rcv_timestamps: false.to_cstring(),
            generate_send_timestamps: false.to_cstring(),
            generate_sender_id: false.to_cstring(),
            generate_sequence_number: false.to_cstring(),
            connect_retries: 0.to_cstring(),
            reconnect_retries: 0.to_cstring(),
            reconnect_retry_wait_ms: 3000.to_cstring(),
            reapply_subscriptions: false.to_cstring(),
        }
    }
}

pub struct SolClient {
    context_p: rsolace_sys::solClient_opaqueContext_pt,
    // context_func_info: rsolace_sys::solClient_context_createFuncInfo_t,
    session_p: rsolace_sys::solClient_opaqueSession_pt,
    session_func_info: Option<rsolace_sys::solClient_session_createFuncInfo_t>,
    rx_msg_callback: Option<fn(&mut Self, SolMsg)>,
    rx_event_callback: Option<fn(&mut Self, SolEvent)>,
}

impl SolClient {
    pub fn new(log_level: SolClientLogLevel) -> Result<SolClient, SolClientError> {
        let mut context_p: rsolace_sys::solClient_opaqueContext_pt = null_mut();
        unsafe {
            rsolace_sys::solClient_initialize(log_level as std::os::raw::c_uint, null_mut());
            let nullptr: *mut std::ffi::c_void = null_mut();
            let mut conext_props: [*const i8; 3] = [
                rsolace_sys::SOLCLIENT_CONTEXT_PROP_CREATE_THREAD.as_ptr() as *const i8,
                rsolace_sys::SOLCLIENT_PROP_ENABLE_VAL.as_ptr() as *const i8,
                null(),
            ];
            let conext_props_ptr: *mut *const i8 = conext_props.as_mut_ptr();

            let mut context_func_info: rsolace_sys::solClient_context_createFuncInfo_t =
                rsolace_sys::solClient_context_createFuncInfo {
                    regFdInfo: rsolace_sys::solClient_context_createRegisterFdFuncInfo {
                        regFdFunc_p: None,
                        unregFdFunc_p: None,
                        user_p: nullptr,
                    },
                };
            let rt_code = rsolace_sys::solClient_context_create(
                conext_props_ptr,
                &mut context_p,
                &mut context_func_info,
                std::mem::size_of::<rsolace_sys::solClient_context_createFuncInfo>(),
            );
            ensure!(
                rt_code == rsolace_sys::solClient_returnCode_SOLCLIENT_OK,
                ContextCreateSnafu
            );

            Ok(SolClient {
                context_p: context_p,
                // context_func_info: context_func_info,
                session_p: null_mut(),
                session_func_info: None,
                rx_msg_callback: None,
                rx_event_callback: None,
            })
        }
    }

    pub fn connect(&mut self, props: SessionProps) -> bool {
        let mut session_props = props.to_c();
        let c = unsafe { std::ffi::CStr::from_ptr(session_props[9]).to_str().unwrap() };
        let uname = unsafe { std::ffi::CStr::from_ptr(session_props[5]).to_str().unwrap() };
        tracing::debug!("cstr: {:?}, {:?}", props.compression_level, c);
        tracing::debug!("username cstr: {:?}, {:?}", props.username, uname);
        let session_props_ptr: *mut *const i8 = session_props.as_mut_ptr();

        let user_p: *mut c_void = self as *mut _ as *mut c_void;

        unsafe extern "C" fn message_receive_callback(
            _opaque_session_p: rsolace_sys::solClient_opaqueSession_pt,
            msg_p: rsolace_sys::solClient_opaqueMsg_pt,
            user_p: *mut std::ffi::c_void,
        ) -> rsolace_sys::solClient_rxMsgCallback_returnCode_t {
            let solmsg = SolMsg::from_ptr(msg_p);
            match solmsg {
                Ok(msg) => {
                    let self_ref: &mut SolClient = &mut *(user_p as *mut SolClient);
                    if let Some(cb) = self_ref.rx_msg_callback {
                        cb(self_ref, msg);
                    } else {
                        msg.dump(true);
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
                    let self_ref: &mut SolClient = &mut *(user_p as *mut SolClient);
                    if let Some(cb) = self_ref.rx_event_callback {
                        cb(self_ref, event)
                    } else {
                        tracing::info!(
                            "event: {}, response code: {}, info: {}",
                            event.get_session_event_string(),
                            event.response_code,
                            event.info
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("event from ptr error: {}", e);
                }
            }
        }

        self.session_func_info = Some(rsolace_sys::solClient_session_createFuncInfo_t {
            rxMsgInfo: rsolace_sys::solClient_session_createRxMsgCallbackFuncInfo_t {
                callback_p: Some(message_receive_callback),
                user_p: user_p,
            },
            eventInfo: rsolace_sys::solClient_session_createEventCallbackFuncInfo_t {
                callback_p: Some(event_receive_callback),
                user_p: user_p,
            },
            rxInfo: rsolace_sys::solClient_session_createRxCallbackFuncInfo {
                callback_p: null_mut(),
                user_p: user_p,
            },
        });
        let session_func_info_ptr: *mut rsolace_sys::solClient_session_createFuncInfo_t =
            &mut self.session_func_info.unwrap();
        unsafe {
            rsolace_sys::solClient_session_create(
                session_props_ptr,
                self.context_p,
                &mut self.session_p,
                session_func_info_ptr,
                std::mem::size_of::<rsolace_sys::solClient_session_createFuncInfo>(),
            );
            let rt_code = rsolace_sys::solClient_session_connect(self.session_p);
            rt_code == (SolClientReturnCode::Ok as i32)
        }
    }

    pub fn disconnect(&mut self) {
        unsafe {
            rsolace_sys::solClient_session_disconnect(self.session_p);
        }
    }

    pub fn set_rx_msg_callback(&mut self, func: fn(&mut Self, SolMsg)) {
        self.rx_msg_callback = Some(func);
    }

    pub fn set_rx_event_callback(&mut self, func: fn(&mut Self, SolEvent)) {
        self.rx_event_callback = Some(func);
    }

    pub fn subscribe(&self, topic: &str) -> SolClientReturnCode {
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code =
                rsolace_sys::solClient_session_topicSubscribe(self.session_p, topic.as_ptr());
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn unsubscribe(&self, topic: &str) -> SolClientReturnCode {
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code =
                rsolace_sys::solClient_session_topicUnsubscribe(self.session_p, topic.as_ptr());
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn subscribe_ext(&self, topic: &str, flag: SolClientSubscribeFlags) -> SolClientReturnCode {
        let topic = CString::new(topic).unwrap();
        unsafe {
            let rt_code = rsolace_sys::solClient_session_topicSubscribeExt(
                self.session_p,
                flag as u32,
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
                self.session_p,
                flag as u32,
                topic.as_ptr(),
            );
            SolClientReturnCode::from_i32(rt_code).unwrap()
        }
    }

    pub fn send_msg(&self, msg: &SolMsg) -> SolClientReturnCode {
        let rt_code =
            unsafe { rsolace_sys::solClient_session_sendMsg(self.session_p, msg.get_ptr()) };
        SolClientReturnCode::from_i32(rt_code).unwrap()
    }

    pub fn send_multiple_msg(&self, msgs: &[SolMsg]) -> SolClientReturnCode {
        let mut arr_msg: [rsolace_sys::solClient_opaqueMsg_pt;
            rsolace_sys::SOLCLIENT_SESSION_SEND_MULTIPLE_LIMIT as usize] =
            [null_mut(); rsolace_sys::SOLCLIENT_SESSION_SEND_MULTIPLE_LIMIT as usize];
        let mut num = 0;
        for (i, msg) in msgs.iter().enumerate() {
            arr_msg[i] = msg.get_ptr();
        }
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendMultipleMsg(
                self.session_p,
                &mut arr_msg as *mut *mut c_void,
                msgs.len() as u32,
                &mut num,
            )
        };
        SolClientReturnCode::from_i32(rt_code).unwrap()
    }

    pub fn send_request(&self, msg: &SolMsg, timeout: u32) -> Result<SolMsg, SolClientError> {
        let mut reply_msg_pt: rsolace_sys::solClient_opaqueMsg_pt = null_mut();
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendRequest(
                self.session_p,
                msg.get_ptr(),
                &mut reply_msg_pt,
                timeout,
            )
        };
        let rt_code = SolClientReturnCode::from_i32(rt_code).unwrap();
        ensure!(
            (timeout > 0 && rt_code == SolClientReturnCode::Ok)
                || (timeout == 0 && rt_code == SolClientReturnCode::InProgress),
            SendRequestSnafu {
                topic: msg.get_topic().context(SolMsgSnafu)?
            }
        );
        // check reply msg when non block
        Ok(SolMsg::from_ptr(reply_msg_pt).unwrap())
    }

    pub fn send_reply(&self, rx_msg: &SolMsg, reply_msg: &SolMsg) -> SolClientReturnCode {
        let rt_code = unsafe {
            rsolace_sys::solClient_session_sendReply(
                self.session_p,
                rx_msg.get_ptr(),
                reply_msg.get_ptr(),
            )
        };
        SolClientReturnCode::from_i32(rt_code).unwrap()
    }

    pub fn modify_client_info(
        &mut self,
        app_description: Option<&str>,
        client_name: Option<&str>,
    ) -> SolClientReturnCode {
        if app_description.is_some() & client_name.is_some() {
            let app_desc = CString::new(app_description.unwrap()).unwrap();
            let client_name = CString::new(client_name.unwrap()).unwrap();
            let mut client_info_props = [
                rsolace_sys::SOLCLIENT_SESSION_PROP_CLIENT_NAME.as_ptr() as *const i8,
                client_name.as_ptr() as *const i8,
                rsolace_sys::SOLCLIENT_SESSION_PROP_APPLICATION_DESCRIPTION.as_ptr() as *const i8,
                app_desc.as_ptr() as *const i8,
                null_mut(),
            ];
            let client_info_props_ptr: *mut *const i8 = client_info_props.as_mut_ptr();
            let rt_code = unsafe {
                rsolace_sys::solClient_session_modifyClientInfo(
                    self.session_p,
                    client_info_props_ptr,
                    rsolace_sys::SOLCLIENT_MODIFYPROP_FLAGS_WAITFORCONFIRM,
                    null_mut(),
                )
            };
            SolClientReturnCode::from_i32(rt_code).unwrap()
        } else if app_description.is_some() {
            let app_desc = CString::new(app_description.unwrap()).unwrap();
            let mut client_info_props = [
                rsolace_sys::SOLCLIENT_SESSION_PROP_APPLICATION_DESCRIPTION.as_ptr() as *const i8,
                app_desc.as_ptr() as *const i8,
                null_mut(),
            ];
            let client_info_props_ptr: *mut *const i8 = client_info_props.as_mut_ptr();
            let rt_code = unsafe {
                rsolace_sys::solClient_session_modifyClientInfo(
                    self.session_p,
                    client_info_props_ptr,
                    rsolace_sys::SOLCLIENT_MODIFYPROP_FLAGS_WAITFORCONFIRM,
                    null_mut(),
                )
            };
            SolClientReturnCode::from_i32(rt_code).unwrap()
        } else if client_name.is_some() {
            let client_name = CString::new(client_name.unwrap()).unwrap();
            let mut client_info_props = [
                rsolace_sys::SOLCLIENT_SESSION_PROP_CLIENT_NAME.as_ptr() as *const i8,
                client_name.as_ptr() as *const i8,
                null_mut(),
            ];
            let client_info_props_ptr: *mut *const i8 = client_info_props.as_mut_ptr();
            let rt_code = unsafe {
                rsolace_sys::solClient_session_modifyClientInfo(
                    self.session_p,
                    client_info_props_ptr,
                    rsolace_sys::SOLCLIENT_MODIFYPROP_FLAGS_WAITFORCONFIRM,
                    null_mut(),
                )
            };
            SolClientReturnCode::from_i32(rt_code).unwrap()
        } else {
            SolClientReturnCode::NotFound
        }
    }
}

impl Drop for SolClient {
    fn drop(&mut self) {
        unsafe {
            rsolace_sys::solClient_context_destroy(&mut self.context_p);
            rsolace_sys::solClient_cleanup();
        }
    }
}
