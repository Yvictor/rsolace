use enum_primitive::*;
use failure::{bail, Error};
use rsolace_sys;
use std::option::Option;
use std::ffi::{CString, CStr, c_void};
use std::ptr::{null, null_mut};
// struct UnSafeSolCliet {
//     context_p: rsolace_sys::solClient_opaqueContext_pt
// }

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SolClientLogLevel {
        EMERGENCY = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_EMERGENCY,
        Alert = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_ALERT,
        Critical = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_CRITICAL,
        Error = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_ERROR,
        Warning = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_WARNING,
        Notice = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_NOTICE,
        Info = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_INFO,
        Debug = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_DEBUG,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(i32)]
    pub enum SolClientReturnCode {
        Ok = rsolace_sys::solClient_returnCode_SOLCLIENT_OK,
        WouldBlock = rsolace_sys::solClient_returnCode_SOLCLIENT_WOULD_BLOCK,
        InProgress = rsolace_sys::solClient_returnCode_SOLCLIENT_IN_PROGRESS,
        NotReady = rsolace_sys::solClient_returnCode_SOLCLIENT_NOT_READY,
        Eos = rsolace_sys::solClient_returnCode_SOLCLIENT_EOS,
        NotFound = rsolace_sys::solClient_returnCode_SOLCLIENT_NOT_FOUND,
        NoEvent = rsolace_sys::solClient_returnCode_SOLCLIENT_NOEVENT,
        InComplete = rsolace_sys::solClient_returnCode_SOLCLIENT_INCOMPLETE,
        Rollback = rsolace_sys::solClient_returnCode_SOLCLIENT_ROLLBACK,
        Fail = rsolace_sys::solClient_returnCode_SOLCLIENT_FAIL,
    }
}


// TODO fn pointer to struct
unsafe extern "C" fn message_receive_callback(
    _opaque_session_p: rsolace_sys::solClient_opaqueSession_pt,
    _msg_p: rsolace_sys::solClient_opaqueMsg_pt,
    _user_p: *mut std::ffi::c_void,
) -> rsolace_sys::solClient_rxMsgCallback_returnCode_t {
    // Implementation of your message receive callback function goes here
    // ...
    // Return appropriate value of solClient_rxMsgCallback_returnCode_t
    println!("rec msg");
    rsolace_sys::solClient_rxMsgCallback_returnCode_SOLCLIENT_CALLBACK_OK
}

unsafe extern "C" fn info_receive_callback(
    _opaque_session_p: rsolace_sys::solClient_opaqueSession_pt,
    event_info_p: rsolace_sys::solClient_session_eventCallbackInfo_pt,
    _user_p: *mut std::ffi::c_void,
) {
    let event_info = *event_info_p;
    let res = rsolace_sys::solClient_session_eventToString(event_info.sessionEvent);
    println!(
        "event: {}, event code: {}, info: {}",
        CStr::from_ptr(res).to_str().unwrap(),
        event_info.responseCode,
        CStr::from_ptr(event_info.info_p)
            .to_str()
            .unwrap(),
    );
}


pub struct SolClient {
    context_p: rsolace_sys::solClient_opaqueContext_pt,
    // context_func_info: rsolace_sys::solClient_context_createFuncInfo_t,
    session_p: rsolace_sys::solClient_opaqueSession_pt,
    session_func_info: Option<rsolace_sys::solClient_session_createFuncInfo_t>,
}

impl SolClient {
    pub fn new(log_level: SolClientLogLevel) -> Result<SolClient, Error> {
        let mut context_p: rsolace_sys::solClient_opaqueContext_pt = null_mut();
        unsafe {
            rsolace_sys::solClient_initialize(
                log_level as std::os::raw::c_uint,
                null_mut(),
            );
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
            })
        }
    }

    pub fn connect(&mut self, host: &str, vpn: &str, username: &str, password: &str, clientname: Option<&str>, _connect_timeout: Option<u32>, compression_level: Option<&str>) -> bool {
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
        let user_p: *mut c_void  = self as *mut _ as *mut c_void;
        self.session_func_info = Some(
            rsolace_sys::solClient_session_createFuncInfo_t {
                rxMsgInfo: rsolace_sys::solClient_session_createRxMsgCallbackFuncInfo_t {
                    callback_p: Some(message_receive_callback),
                    user_p: user_p,
                },
                eventInfo: rsolace_sys::solClient_session_createEventCallbackFuncInfo_t {
                    callback_p: Some(info_receive_callback),
                    user_p: user_p,
                },
                rxInfo: rsolace_sys::solClient_session_createRxCallbackFuncInfo {
                    callback_p: null_mut(),
                    user_p: user_p,
                },
            }
        );
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

    pub fn disconnect(&self) {}
}

impl Drop for SolClient {
    fn drop(&mut self) {
        unsafe {
            rsolace_sys::solClient_cleanup();
        }
    }
}
