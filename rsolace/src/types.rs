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

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SolClientSessionEvent {
        UpNotice = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_UP_NOTICE,
        DownError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_DOWN_ERROR,
        ConnectFailedError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_CONNECT_FAILED_ERROR,
        RejectedMsgError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_REJECTED_MSG_ERROR,
        SubscriptionError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_SUBSCRIPTION_ERROR,
        RxMsgTooBigError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_RX_MSG_TOO_BIG_ERROR,
        Acknowledgement = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_ACKNOWLEDGEMENT,
        AssuredPublishingUp = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_ASSURED_PUBLISHING_UP,
        AssuredConnectFailed = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_ASSURED_CONNECT_FAILED,
        // AssuredDeliveryDown = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_ASSURED_DELIVERY_DOWN,
        TeUnsubscribeError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_TE_UNSUBSCRIBE_ERROR,
        // DteUnsubscribeError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_DTE_UNSUBSCRIBE_ERROR,
        TeUnsubscribeOk = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_TE_UNSUBSCRIBE_OK,
        // DteUnsubscribeOk = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_DTE_UNSUBSCRIBE_OK,
        CanSend = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_CAN_SEND,
        ReconnectingNotice = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_RECONNECTING_NOTICE,
        ReconnectedNotice = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_RECONNECTED_NOTICE,
        ProvisionError = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_PROVISION_ERROR,
        ProvisionOk = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_PROVISION_OK,
        SubscriptionOk = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_SUBSCRIPTION_OK,
        VirtualRouterNameChanged = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_VIRTUAL_ROUTER_NAME_CHANGED,
        ModifyPropOk = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_MODIFYPROP_OK,
        ModifyPropFail = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_MODIFYPROP_FAIL,
        RepublishUnackedMessages = rsolace_sys::solClient_session_event_SOLCLIENT_SESSION_EVENT_REPUBLISH_UNACKED_MESSAGES,
    }

}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SolClientSubscribeFlags {
        WaitForConfirm = rsolace_sys::SOLCLIENT_SUBSCRIBE_FLAGS_WAITFORCONFIRM,
        LocalDispatchOnly = rsolace_sys::SOLCLIENT_SUBSCRIBE_FLAGS_LOCAL_DISPATCH_ONLY,
        RequestConfirm = rsolace_sys::SOLCLIENT_SUBSCRIBE_FLAGS_REQUEST_CONFIRM,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum SolClientDeliveryMode {
        Direct = rsolace_sys::SOLCLIENT_DELIVERY_MODE_DIRECT,
        Persistent = rsolace_sys::SOLCLIENT_DELIVERY_MODE_PERSISTENT,
        NonPersistent = rsolace_sys::SOLCLIENT_DELIVERY_MODE_NONPERSISTENT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(i32)]
    pub enum SolClientDestType {
        Null = rsolace_sys::solClient_destinationType_SOLCLIENT_NULL_DESTINATION,
        Topic = rsolace_sys::solClient_destinationType_SOLCLIENT_TOPIC_DESTINATION,
        Queue = rsolace_sys::solClient_destinationType_SOLCLIENT_QUEUE_DESTINATION,
        TopicTemp = rsolace_sys::solClient_destinationType_SOLCLIENT_TOPIC_TEMP_DESTINATION,
        QueueTemp = rsolace_sys::solClient_destinationType_SOLCLIENT_QUEUE_TEMP_DESTINATION,
    }
}
