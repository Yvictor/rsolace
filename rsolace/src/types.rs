use enum_primitive::*;


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