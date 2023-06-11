use super::solevent::SolEvent;
use super::solmsg::SolMsg;
use super::types::{SolClientLogLevel, SolClientReturnCode, SolClientSubscribeFlags};
use enum_primitive::FromPrimitive;
use failure::{bail, Error};
use rsolace_sys;
use std::ffi::{c_void, CString};
use std::option::Option;
use std::ptr::{null, null_mut};

// TODO fn pointer to struct

pub struct SolClient {
    context_p: rsolace_sys::solClient_opaqueContext_pt,
    // context_func_info: rsolace_sys::solClient_context_createFuncInfo_t,
    session_p: rsolace_sys::solClient_opaqueSession_pt,
    session_func_info: Option<rsolace_sys::solClient_session_createFuncInfo_t>,
    rx_msg_callback: Option<fn(SolMsg)>,
    rx_event_callback: Option<fn(SolEvent)>,
}

impl SolClient {
    pub fn new(log_level: SolClientLogLevel) -> Result<SolClient, Error> {
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
            if rt_code != rsolace_sys::solClient_returnCode_SOLCLIENT_OK {
                bail!("solcient error"); // error info
            }
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

    pub fn connect(
        &mut self,
        host: &str,
        vpn: &str,
        username: &str,
        password: &str,
        clientname: Option<&str>,
        _connect_timeout: Option<u32>,
        compression_level: Option<&str>,
    ) -> bool {
        let host = CString::new(host).unwrap();
        let vpn = CString::new(vpn).unwrap();
        let username = CString::new(username).unwrap();
        let password = CString::new(password).unwrap();
        let _clientname = CString::new(clientname.unwrap_or("")).unwrap();
        let compression_level = CString::new(compression_level.unwrap_or("1")).unwrap();
        let mut session_props = [
            rsolace_sys::SOLCLIENT_SESSION_PROP_HOST.as_ptr() as *const i8,
            host.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_VPN_NAME.as_ptr() as *const i8,
            vpn.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_USERNAME.as_ptr() as *const i8,
            username.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_PASSWORD.as_ptr() as *const i8,
            password.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_COMPRESSION_LEVEL.as_ptr() as *const i8,
            compression_level.as_ptr() as *const i8,
            null(),
        ];
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
                        cb(msg);
                    } else {
                        msg.dump(true);
                    }
                }
                Err(e) => {
                    tracing::error!("msg from ptr error: {}", e);
                }
            }
            println!("rec msg");
            rsolace_sys::solClient_rxMsgCallback_returnCode_SOLCLIENT_CALLBACK_OK
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
                        cb(event)
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

    pub fn set_rx_msg_callback(&mut self, func: fn(SolMsg)) {
        self.rx_msg_callback = Some(func);
    }

    pub fn set_rx_event_callback(&mut self, func: fn(SolEvent)) {
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

    pub fn unsubscribe_ext(&self, topic: &str, flag: SolClientSubscribeFlags) -> SolClientReturnCode {
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
}

impl Drop for SolClient {
    fn drop(&mut self) {
        unsafe {
            rsolace_sys::solClient_context_destroy(&mut self.context_p);
            rsolace_sys::solClient_cleanup();
        }
    }
}
