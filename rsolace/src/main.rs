use rsolace_sys;
use std::env;
use std::ptr;
use std::{thread, time};
use tracing;

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
    tracing::info!(
        "event: {}, event code: {}, info: {}",
        std::ffi::CStr::from_ptr(res).to_str().unwrap(),
        event_info.responseCode,
        std::ffi::CStr::from_ptr(event_info.info_p)
            .to_str()
            .unwrap(),
    );
}

// struct SolClient {
//     context_p: rsolace_sys::solClient_opaqueContext_pt
// }

fn main() {
    let solclient_lib_dir = "rsolace-sys/solclient-7.25.0.10/lib";
    env::set_var("LD_LIBRARY_PATH", solclient_lib_dir);
    tracing_subscriber::fmt::init();
    let use_p: *mut std::ffi::c_void = ptr::null_mut();
    println!("Hello, world!");
    unsafe {
        let mut context_pt: rsolace_sys::solClient_opaqueContext_pt = ptr::null_mut();
        let mut context_func_info: rsolace_sys::solClient_context_createFuncInfo_t =
            rsolace_sys::solClient_context_createFuncInfo {
                regFdInfo: rsolace_sys::solClient_context_createRegisterFdFuncInfo {
                    regFdFunc_p: None,
                    unregFdFunc_p: None,
                    user_p: use_p,
                },
            };

        println!(
            "context func info size {}",
            std::mem::size_of_val(&context_func_info)
        );
        let mut session_p: rsolace_sys::solClient_opaqueSession_pt = ptr::null_mut();
        let mut session_func_info: rsolace_sys::solClient_session_createFuncInfo_t =
            rsolace_sys::solClient_session_createFuncInfo_t {
                rxMsgInfo: rsolace_sys::solClient_session_createRxMsgCallbackFuncInfo_t {
                    callback_p: Some(message_receive_callback),
                    user_p: use_p,
                },
                eventInfo: rsolace_sys::solClient_session_createEventCallbackFuncInfo_t {
                    callback_p: Some(info_receive_callback),
                    user_p: use_p,
                },
                rxInfo: rsolace_sys::solClient_session_createRxCallbackFuncInfo {
                    callback_p: ptr::null_mut(),
                    user_p: use_p,
                },
            };
        let session_func_info_ptr: *mut rsolace_sys::solClient_session_createFuncInfo_t =
            &mut session_func_info;

        const DEFAULT_CREATE_THREAD: &[u8] = b"1\0";
        let mut conext_props: [*const i8; 3] = [
            rsolace_sys::SOLCLIENT_CONTEXT_PROP_CREATE_THREAD.as_ptr() as *const i8,
            DEFAULT_CREATE_THREAD.as_ptr() as *const i8,
            std::ptr::null(),
        ];
        let conext_props_ptr: *mut *const i8 = conext_props.as_mut_ptr();

        const HOST: &[u8] = b"218.32.76.102:80\0";
        const VPN: &[u8] = b"sinopac\0";
        const USER: &[u8] = b"shioaji\0";
        const PASSWORD: &[u8] = b"shioaji111\0";
        const COMPRESSION_LEVEL: &[u8] = b"5\0";
        const CONNECT_TIMEOUT: &[u8] = b"500\0";
        // let host = std::ffi::CString::new("203.66.91.161:80").unwrap();
        // let vpn = std::ffi::CString::new("sinopac").unwrap();
        // let user = std::ffi::CString::new("shioaji").unwrap();
        // let pasword = std::ffi::CString::new("sahjiio111").unwrap();
        // const SESSION_PROPS_SIZE: usize = 20;
        let mut session_props: [*const i8; 17] = [
            rsolace_sys::SOLCLIENT_SESSION_PROP_HOST.as_ptr() as *const i8,
            HOST.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_VPN_NAME.as_ptr() as *const i8,
            VPN.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_USERNAME.as_ptr() as *const i8,
            USER.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_PASSWORD.as_ptr() as *const i8,
            PASSWORD.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_COMPRESSION_LEVEL.as_ptr() as *const i8,
            COMPRESSION_LEVEL.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_GENERATE_SEQUENCE_NUMBER.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_PROP_ENABLE_VAL.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_TCP_NODELAY.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_PROP_ENABLE_VAL.as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_CONNECT_TIMEOUT_MS.as_ptr() as *const i8,
            CONNECT_TIMEOUT.as_ptr() as *const i8,
            // rsolace_sys::SOLCLIENT_SESSION_PROP_CONNECT_BLOCKING.as_ptr() as *const i8,
            // rsolace_sys::SOLCLIENT_PROP_DISABLE_VAL.as_ptr() as *const i8,
            std::ptr::null(),
        ];
        let session_props_ptr: *mut *const i8 = session_props.as_mut_ptr();

        rsolace_sys::solClient_initialize(
            rsolace_sys::solClient_log_level_SOLCLIENT_LOG_NOTICE,
            ptr::null_mut(),
        );

        rsolace_sys::solClient_context_create(
            conext_props_ptr, //default_create_thread,
            &mut context_pt,
            &mut context_func_info,
            std::mem::size_of::<rsolace_sys::solClient_context_createFuncInfo>(),
        );

        rsolace_sys::solClient_session_create(
            session_props_ptr,
            context_pt,
            &mut session_p,
            session_func_info_ptr,
            std::mem::size_of::<rsolace_sys::solClient_session_createFuncInfo>(),
        );

        let rt_code = rsolace_sys::solClient_session_connect(session_p);
        let res = rsolace_sys::solClient_returnCodeToString(rt_code);
        tracing::info!(
            "connect return code: {}, {}",
            rt_code,
            std::ffi::CStr::from_ptr(res).to_str().unwrap()
        );

        thread::sleep(time::Duration::from_secs(10));

        rsolace_sys::solClient_cleanup();
    }
}
