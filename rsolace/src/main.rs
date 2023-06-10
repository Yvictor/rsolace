use rsolace_sys;
use std::env;
use std::ptr;
use std::{thread, time};

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
    _event_info_p: rsolace_sys::solClient_session_eventCallbackInfo_pt,
    _user_p: *mut std::ffi::c_void,
) {
    println!("rec info");
}

fn main() {
    let solclient_lib_dir = "rsolace-sys/solclient-7.25.0.10/lib";
    env::set_var("LD_LIBRARY_PATH", solclient_lib_dir);
    let use_p: *mut std::ffi::c_void = ptr::null_mut();
    println!("Hello, world!");
    unsafe {
        let mut context_pt: *mut ::std::os::raw::c_void = ptr::null_mut();
        let mut context_func_info: rsolace_sys::solClient_context_createFuncInfo_t =
            rsolace_sys::solClient_context_createFuncInfo {
                regFdInfo: rsolace_sys::solClient_context_createRegisterFdFuncInfo {
                    regFdFunc_p: None,
                    unregFdFunc_p: None,
                    user_p: use_p,
                },
            };
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
        // session_func_info
        let d: *const std::os::raw::c_char =
            (rsolace_sys::SOLCLIENT_CONTEXT_PROP_DEFAULT_CREATE_THREAD as *const u8)
                as *const std::os::raw::c_char;
        // std::mem::transmute::<*const u8, *const std::os::raw::c_char>(
        //     rsolace_sys::SOLCLIENT_CONTEXT_PROP_DEFAULT_CREATE_THREAD as *const u8,
        // );
        let default_create_thread: *mut *const std::os::raw::c_char =
            d as *mut *const std::os::raw::c_char;
        // const SESSION_PROPS_SIZE: usize = 20;
        let mut session_props: [*const i8; 11] = [
            rsolace_sys::SOLCLIENT_SESSION_PROP_HOST.as_ptr() as *const i8,
            "203.66.91.161:80".as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_VPN_NAME.as_ptr() as *const i8,
            "sinopac".as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_USERNAME.as_ptr() as *const i8,
            "shioaji".as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_PASSWORD.as_ptr() as *const i8,
            "sahjiio111".as_ptr() as *const i8,
            rsolace_sys::SOLCLIENT_SESSION_PROP_COMPRESSION_LEVEL.as_ptr() as *const i8,
            "1".as_ptr() as *const i8,
            std::ptr::null(),
        ];
        let session_props_ptr: *mut *const i8 = session_props.as_mut_ptr();

        rsolace_sys::solClient_initialize(
            rsolace_sys::solClient_log_level_SOLCLIENT_LOG_NOTICE,
            ptr::null_mut(),
        );
        rsolace_sys::solClient_context_create(
            default_create_thread,
            &mut context_pt,
            &mut context_func_info,
            std::mem::size_of::<*const std::ffi::c_void>(),
        );

        rsolace_sys::solClient_session_create(
            session_props_ptr,
            context_pt,
            &mut session_p,
            session_func_info_ptr,
            std::mem::size_of::<*const std::ffi::c_void>(),
        );

        rsolace_sys::solClient_session_connect(session_p);
        println!("Connected.");

        thread::sleep(time::Duration::from_secs(5));

        rsolace_sys::solClient_cleanup();
    }
}
