use enum_primitive::*;

#[cfg(target_os = "windows")]
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(i32)]
    pub enum SolClientLogLevel {
        Emergency = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_EMERGENCY,
        Alert = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_ALERT,
        Critical = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_CRITICAL,
        Error = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_ERROR,
        Warning = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_WARNING,
        Notice = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_NOTICE,
        Info = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_INFO,
        Debug = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_DEBUG,
    }
}

#[cfg(not(target_os = "windows"))]
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum SolClientLogLevel {
        Emergency = rsolace_sys::solClient_log_level_SOLCLIENT_LOG_EMERGENCY,
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
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

#[cfg(target_os = "windows")]
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(i32)]
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

#[cfg(not(target_os = "windows"))]
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

impl From<u32> for SolClientSessionEvent {
    fn from(value: u32) -> Self {
        SolClientSessionEvent::from_u32(value).unwrap()
    }
}

impl From<i32> for SolClientSessionEvent {
    fn from(value: i32) -> Self {
        SolClientSessionEvent::from_u32(value as u32).unwrap()
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum SolClientSubscribeFlags {
        WaitForConfirm = rsolace_sys::SOLCLIENT_SUBSCRIBE_FLAGS_WAITFORCONFIRM,
        LocalDispatchOnly = rsolace_sys::SOLCLIENT_SUBSCRIBE_FLAGS_LOCAL_DISPATCH_ONLY,
        RequestConfirm = rsolace_sys::SOLCLIENT_SUBSCRIBE_FLAGS_REQUEST_CONFIRM,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum SolClientCacheRequestFlags {
        LiveDataFlowThru = rsolace_sys::SOLCLIENT_CACHEREQUEST_FLAGS_LIVEDATA_FLOWTHRU,
        LiveDataFulfill = rsolace_sys::SOLCLIENT_CACHEREQUEST_FLAGS_LIVEDATA_FULFILL,
        LiveDataQueue = rsolace_sys::SOLCLIENT_CACHEREQUEST_FLAGS_LIVEDATA_QUEUE,
        NoSubscribe = rsolace_sys::SOLCLIENT_CACHEREQUEST_FLAGS_NO_SUBSCRIBE,
        NowaitReply = rsolace_sys::SOLCLIENT_CACHEREQUEST_FLAGS_NOWAIT_REPLY,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(i32)]
    pub enum SolClientCacheStatus {
        Invalid = rsolace_sys::solClient_cacheStatus_SOLCLIENT_CACHE_INVALID_MESSAGE,
        Live = rsolace_sys::solClient_cacheStatus_SOLCLIENT_CACHE_LIVE_MESSAGE,
        Cache = rsolace_sys::solClient_cacheStatus_SOLCLIENT_CACHE_MESSAGE,
        Suspect = rsolace_sys::solClient_cacheStatus_SOLCLIENT_CACHE_SUSPECT_MESSAGE,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum SolClientDeliveryMode {
        Direct = rsolace_sys::SOLCLIENT_DELIVERY_MODE_DIRECT,
        Persistent = rsolace_sys::SOLCLIENT_DELIVERY_MODE_PERSISTENT,
        NonPersistent = rsolace_sys::SOLCLIENT_DELIVERY_MODE_NONPERSISTENT,
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(i32)]
    pub enum SolClientDestType {
        Null = rsolace_sys::solClient_destinationType_SOLCLIENT_NULL_DESTINATION,
        Topic = rsolace_sys::solClient_destinationType_SOLCLIENT_TOPIC_DESTINATION,
        Queue = rsolace_sys::solClient_destinationType_SOLCLIENT_QUEUE_DESTINATION,
        TopicTemp = rsolace_sys::solClient_destinationType_SOLCLIENT_TOPIC_TEMP_DESTINATION,
        QueueTemp = rsolace_sys::solClient_destinationType_SOLCLIENT_QUEUE_TEMP_DESTINATION,
    }
}

#[cfg(target_os = "windows")]
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(i32)]
    pub enum SolClientSubCode {
        Ok = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OK,
        ParamOutOfRange = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PARAM_OUT_OF_RANGE,
        ParamNullPtr = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PARAM_NULL_PTR,
        ParamConflict = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PARAM_CONFLICT,
        InsufficientSpace = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INSUFFICIENT_SPACE,
        OutOfResources = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OUT_OF_RESOURCES,
        InternalError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INTERNAL_ERROR,
        OutOfMemory = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OUT_OF_MEMORY,
        ProtocolError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PROTOCOL_ERROR,
        InitNotCalled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INIT_NOT_CALLED,
        Timeout = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TIMEOUT,
        KeepAliveFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_KEEP_ALIVE_FAILURE,
        SessionNotEstablished = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SESSION_NOT_ESTABLISHED,
        OsError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OS_ERROR,
        CommunicationError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMMUNICATION_ERROR,
        UserDataTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_USER_DATA_TOO_LARGE,
        TopicTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOPIC_TOO_LARGE,
        InvalidTopicSyntax = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TOPIC_SYNTAX,
        XmlParseError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_XML_PARSE_ERROR,
        LoginFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LOGIN_FAILURE,
        InvalidVirtualAddress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_VIRTUAL_ADDRESS,
        ClientDeleteInProgress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_DELETE_IN_PROGRESS,
        TooManyClients = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOO_MANY_CLIENTS,
        SubscriptionAlreadyPresent = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_ALREADY_PRESENT,
        SubscriptionNotFound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_NOT_FOUND,
        SubscriptionInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_INVALID,
        SubscriptionOther = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_OTHER,
        ControlOther = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CONTROL_OTHER,
        DataOther = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DATA_OTHER,
        LogFileError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LOG_FILE_ERROR,
        MessageTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_TOO_LARGE,
        SubscriptionTooMany = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_TOO_MANY,
        InvalidSessionOperation = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_SESSION_OPERATION,
        TopicMissing = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOPIC_MISSING,
        AssuredMessagingNotEstablished = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ASSURED_MESSAGING_NOT_ESTABLISHED,
        AssuredMessagingStateError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ASSURED_MESSAGING_STATE_ERROR,
        QueueNameTopicConflict = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUENAME_TOPIC_CONFLICT,
        QueueNameTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUENAME_TOO_LARGE,
        QueueNameInvalidMode = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUENAME_INVALID_MODE,
        MaxTotalMsgSizeExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_TOTAL_MSGSIZE_EXCEEDED,
        DBlockAlreadyExists = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DBLOCK_ALREADY_EXISTS,
        NoStructuredData = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_STRUCTURED_DATA,
        ContainerBusy = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CONTAINER_BUSY,
        InvalidDataConversion = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_DATA_CONVERSION,
        CannotModifyWhileNotIdle = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CANNOT_MODIFY_WHILE_NOT_IDLE,
        MsgVpnNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MSG_VPN_NOT_ALLOWED,
        ClientNameInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_NAME_INVALID,
        MsgVpnUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MSG_VPN_UNAVAILABLE,
        ClientUsernameIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_USERNAME_IS_SHUTDOWN,
        DynamicClientsNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DYNAMIC_CLIENTS_NOT_ALLOWED,
        ClientNameAlreadyInUse = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_NAME_ALREADY_IN_USE,
        CacheNoData = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_NO_DATA,
        CacheSuspectData = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_SUSPECT_DATA,
        CacheErrorResponse = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_ERROR_RESPONSE,
        CacheInvalidSession = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_INVALID_SESSION,
        CacheTimeout = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_TIMEOUT,
        CacheLiveDataFulfill = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_LIVEDATA_FULFILL,
        CacheAlreadyInProgress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_ALREADY_IN_PROGRESS,
        MissingReplyTo = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MISSING_REPLY_TO,
        CannotBindToQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CANNOT_BIND_TO_QUEUE,
        InvalidTopicNameForTe = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TOPIC_NAME_FOR_TE,
        UnknownQueueName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_QUEUE_NAME,
        UnknownTeName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_TE_NAME,
        MaxClientsForQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_CLIENTS_FOR_QUEUE,
        MaxClientsForTe = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_CLIENTS_FOR_TE,
        UnexpectedUnbind = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNEXPECTED_UNBIND,
        QueueNotFound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUE_NOT_FOUND,
        ClientAclDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_ACL_DENIED,
        SubscriptionAclDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_ACL_DENIED,
        PublishAclDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PUBLISH_ACL_DENIED,
        DeliverToOneInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DELIVER_TO_ONE_INVALID,
        SpoolOverQuota = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SPOOL_OVER_QUOTA,
        QueueShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUE_SHUTDOWN,
        TeShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TE_SHUTDOWN,
        NoMoreNonDurableQueueOrTe = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_MORE_NON_DURABLE_QUEUE_OR_TE,
        EndpointAlreadyExists = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ENDPOINT_ALREADY_EXISTS,
        PermissionNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PERMISSION_NOT_ALLOWED,
        InvalidSelector = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_SELECTOR,
        MaxMessageUsageExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_MESSAGE_USAGE_EXCEEDED,
        EndpointPropertyMismatch = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ENDPOINT_PROPERTY_MISMATCH,
        SubscriptionManagerDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_MANAGER_DENIED,
        UnknownClientName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_CLIENT_NAME,
        QuotaOutOfRange = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUOTA_OUT_OF_RANGE,
        SubscriptionAttributesConflict = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_ATTRIBUTES_CONFLICT,
        InvalidSmfMessage = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_SMF_MESSAGE,
        NoLocalNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_LOCAL_NOT_SUPPORTED,
        UnsubscribeNotAllowedClientsBound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNSUBSCRIBE_NOT_ALLOWED_CLIENTS_BOUND,
        CannotBlockInContext = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CANNOT_BLOCK_IN_CONTEXT,
        FlowActiveFlowIndicationUnsupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FLOW_ACTIVE_FLOW_INDICATION_UNSUPPORTED,
        UnresolvedHost = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNRESOLVED_HOST,
        CutThroughUnsupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CUT_THROUGH_UNSUPPORTED,
        CutThroughAlreadyBound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CUT_THROUGH_ALREADY_BOUND,
        CutThroughIncompatibleWithSession = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CUT_THROUGH_INCOMPATIBLE_WITH_SESSION,
        InvalidFlowOperation = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_FLOW_OPERATION,
        UnknownFlowName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_FLOW_NAME,
        ReplicationIsStandby = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLICATION_IS_STANDBY,
        LowPriorityMsgCongestion = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LOW_PRIORITY_MSG_CONGESTION,
        LibraryNotLoaded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LIBRARY_NOT_LOADED,
        FailedLoadingTruststore = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FAILED_LOADING_TRUSTSTORE,
        UntrustedCertificate = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNTRUSTED_CERTIFICATE,
        UntrustedCommonname = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNTRUSTED_COMMONNAME,
        CertificateDateInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CERTIFICATE_DATE_INVALID,
        FailedLoadingCertificateAndKey = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FAILED_LOADING_CERTIFICATE_AND_KEY,
        BasicAuthenticationIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_BASIC_AUTHENTICATION_IS_SHUTDOWN,
        ClientCertificateAuthenticationIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_CERTIFICATE_AUTHENTICATION_IS_SHUTDOWN,
        UntrustedClientCertificate = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNTRUSTED_CLIENT_CERTIFICATE,
        ClientCertificateDateInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_CERTIFICATE_DATE_INVALID,
        CacheRequestCancelled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_REQUEST_CANCELLED,
        DeliveryModeUnsupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DELIVERY_MODE_UNSUPPORTED,
        PublisherNotCreated = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PUBLISHER_NOT_CREATED,
        FlowUnbound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FLOW_UNBOUND,
        InvalidTransactedSessionId = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TRANSACTED_SESSION_ID,
        InvalidTransactionId = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TRANSACTION_ID,
        MaxTransactedSessionsExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_TRANSACTED_SESSIONS_EXCEEDED,
        TransactedSessionNameInUse = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TRANSACTED_SESSION_NAME_IN_USE,
        ServiceUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SERVICE_UNAVAILABLE,
        NoTransactionStarted = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_TRANSACTION_STARTED,
        PublisherNotEstablished = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PUBLISHER_NOT_ESTABLISHED,
        MessagePublishFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_PUBLISH_FAILURE,
        TransactionFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TRANSACTION_FAILURE,
        MessageConsumeFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_CONSUME_FAILURE,
        EndpointModified = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ENDPOINT_MODIFIED,
        InvalidConnectionOwner = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_CONNECTION_OWNER,
        KerberosAuthenticationIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_KERBEROS_AUTHENTICATION_IS_SHUTDOWN,
        CommitOrRollbackInProgress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMMIT_OR_ROLLBACK_IN_PROGRESS,
        UnbindResponseLost = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNBIND_RESPONSE_LOST,
        MaxTransactionsExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_TRANSACTIONS_EXCEEDED,
        CommitStatusUnknown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMMIT_STATUS_UNKNOWN,
        ProxyAuthRequired = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PROXY_AUTH_REQUIRED,
        ProxyAuthFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PROXY_AUTH_FAILURE,
        NoSubscriptionMatch = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_SUBSCRIPTION_MATCH,
        SubscriptionMatchError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_MATCH_ERROR,
        SelectorMatchError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SELECTOR_MATCH_ERROR,
        ReplayNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_NOT_SUPPORTED,
        ReplayDisabled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_DISABLED,
        ClientInitiatedReplayNonExclusiveNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_INITIATED_REPLAY_NON_EXCLUSIVE_NOT_ALLOWED,
        ClientInitiatedReplayInactiveFlowNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_INITIATED_REPLAY_INACTIVE_FLOW_NOT_ALLOWED,
        ClientInitiatedReplayBrowserFlowNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_INITIATED_REPLAY_BROWSER_FLOW_NOT_ALLOWED,
        ReplayTemporaryNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_TEMPORARY_NOT_SUPPORTED,
        UnknownStartLocationType = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_START_LOCATION_TYPE,
        ReplayMessageUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_MESSAGE_UNAVAILABLE,
        ReplayStarted = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_STARTED,
        ReplayCancelled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_CANCELLED,
        ReplayStartTimeNotAvailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_START_TIME_NOT_AVAILABLE,
        ReplayMessageRejected = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_MESSAGE_REJECTED,
        ReplayLogModified = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_LOG_MODIFIED,
        MismatchedEndpointErrorId = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MISMATCHED_ENDPOINT_ERROR_ID,
        OutOfReplayResources = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OUT_OF_REPLAY_RESOURCES,
        TopicOrSelectorModifiedOnDurableTopicEndpoint = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOPIC_OR_SELECTOR_MODIFIED_ON_DURABLE_TOPIC_ENDPOINT,
        ReplayFailed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_FAILED,
        CompressedSslNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMPRESSED_SSL_NOT_SUPPORTED,
        SharedSubscriptionsNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SHARED_SUBSCRIPTIONS_NOT_SUPPORTED,
        SharedSubscriptionsNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SHARED_SUBSCRIPTIONS_NOT_ALLOWED,
        SharedSubscriptionsEndpointNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SHARED_SUBSCRIPTIONS_ENDPOINT_NOT_ALLOWED,
        ObjectDestroyed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OBJECT_DESTROYED,
        DeliveryCountNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DELIVERY_COUNT_NOT_SUPPORTED,
        ReplayStartMessageUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_START_MESSAGE_UNAVAILABLE,
        MessageIdNotComparable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_ID_NOT_COMPARABLE,
        ReplayAnonymousNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_ANONYMOUS_NOT_SUPPORTED,
        BrowsingNotSupportedOnPartitionedQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_BROWSING_NOT_SUPPORTED_ON_PARTITIONED_QUEUE,
        SelectorsNotSupportedOnPartitionedQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SELECTORS_NOT_SUPPORTED_ON_PARTITIONED_QUEUE,
    }
}

#[cfg(not(target_os = "windows"))]
enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    #[repr(u32)]
    pub enum SolClientSubCode {
        Ok = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OK,
        ParamOutOfRange = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PARAM_OUT_OF_RANGE,
        ParamNullPtr = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PARAM_NULL_PTR,
        ParamConflict = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PARAM_CONFLICT,
        InsufficientSpace = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INSUFFICIENT_SPACE,
        OutOfResources = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OUT_OF_RESOURCES,
        InternalError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INTERNAL_ERROR,
        OutOfMemory = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OUT_OF_MEMORY,
        ProtocolError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PROTOCOL_ERROR,
        InitNotCalled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INIT_NOT_CALLED,
        Timeout = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TIMEOUT,
        KeepAliveFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_KEEP_ALIVE_FAILURE,
        SessionNotEstablished = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SESSION_NOT_ESTABLISHED,
        OsError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OS_ERROR,
        CommunicationError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMMUNICATION_ERROR,
        UserDataTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_USER_DATA_TOO_LARGE,
        TopicTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOPIC_TOO_LARGE,
        InvalidTopicSyntax = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TOPIC_SYNTAX,
        XmlParseError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_XML_PARSE_ERROR,
        LoginFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LOGIN_FAILURE,
        InvalidVirtualAddress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_VIRTUAL_ADDRESS,
        ClientDeleteInProgress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_DELETE_IN_PROGRESS,
        TooManyClients = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOO_MANY_CLIENTS,
        SubscriptionAlreadyPresent = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_ALREADY_PRESENT,
        SubscriptionNotFound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_NOT_FOUND,
        SubscriptionInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_INVALID,
        SubscriptionOther = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_OTHER,
        ControlOther = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CONTROL_OTHER,
        DataOther = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DATA_OTHER,
        LogFileError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LOG_FILE_ERROR,
        MessageTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_TOO_LARGE,
        SubscriptionTooMany = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_TOO_MANY,
        InvalidSessionOperation = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_SESSION_OPERATION,
        TopicMissing = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOPIC_MISSING,
        AssuredMessagingNotEstablished = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ASSURED_MESSAGING_NOT_ESTABLISHED,
        AssuredMessagingStateError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ASSURED_MESSAGING_STATE_ERROR,
        QueueNameTopicConflict = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUENAME_TOPIC_CONFLICT,
        QueueNameTooLarge = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUENAME_TOO_LARGE,
        QueueNameInvalidMode = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUENAME_INVALID_MODE,
        MaxTotalMsgSizeExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_TOTAL_MSGSIZE_EXCEEDED,
        DBlockAlreadyExists = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DBLOCK_ALREADY_EXISTS,
        NoStructuredData = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_STRUCTURED_DATA,
        ContainerBusy = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CONTAINER_BUSY,
        InvalidDataConversion = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_DATA_CONVERSION,
        CannotModifyWhileNotIdle = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CANNOT_MODIFY_WHILE_NOT_IDLE,
        MsgVpnNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MSG_VPN_NOT_ALLOWED,
        ClientNameInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_NAME_INVALID,
        MsgVpnUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MSG_VPN_UNAVAILABLE,
        ClientUsernameIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_USERNAME_IS_SHUTDOWN,
        DynamicClientsNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DYNAMIC_CLIENTS_NOT_ALLOWED,
        ClientNameAlreadyInUse = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_NAME_ALREADY_IN_USE,
        CacheNoData = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_NO_DATA,
        CacheSuspectData = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_SUSPECT_DATA,
        CacheErrorResponse = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_ERROR_RESPONSE,
        CacheInvalidSession = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_INVALID_SESSION,
        CacheTimeout = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_TIMEOUT,
        CacheLiveDataFulfill = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_LIVEDATA_FULFILL,
        CacheAlreadyInProgress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_ALREADY_IN_PROGRESS,
        MissingReplyTo = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MISSING_REPLY_TO,
        CannotBindToQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CANNOT_BIND_TO_QUEUE,
        InvalidTopicNameForTe = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TOPIC_NAME_FOR_TE,
        UnknownQueueName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_QUEUE_NAME,
        UnknownTeName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_TE_NAME,
        MaxClientsForQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_CLIENTS_FOR_QUEUE,
        MaxClientsForTe = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_CLIENTS_FOR_TE,
        UnexpectedUnbind = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNEXPECTED_UNBIND,
        QueueNotFound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUE_NOT_FOUND,
        ClientAclDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_ACL_DENIED,
        SubscriptionAclDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_ACL_DENIED,
        PublishAclDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PUBLISH_ACL_DENIED,
        DeliverToOneInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DELIVER_TO_ONE_INVALID,
        SpoolOverQuota = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SPOOL_OVER_QUOTA,
        QueueShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUEUE_SHUTDOWN,
        TeShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TE_SHUTDOWN,
        NoMoreNonDurableQueueOrTe = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_MORE_NON_DURABLE_QUEUE_OR_TE,
        EndpointAlreadyExists = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ENDPOINT_ALREADY_EXISTS,
        PermissionNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PERMISSION_NOT_ALLOWED,
        InvalidSelector = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_SELECTOR,
        MaxMessageUsageExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_MESSAGE_USAGE_EXCEEDED,
        EndpointPropertyMismatch = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ENDPOINT_PROPERTY_MISMATCH,
        SubscriptionManagerDenied = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_MANAGER_DENIED,
        UnknownClientName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_CLIENT_NAME,
        QuotaOutOfRange = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_QUOTA_OUT_OF_RANGE,
        SubscriptionAttributesConflict = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_ATTRIBUTES_CONFLICT,
        InvalidSmfMessage = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_SMF_MESSAGE,
        NoLocalNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_LOCAL_NOT_SUPPORTED,
        UnsubscribeNotAllowedClientsBound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNSUBSCRIBE_NOT_ALLOWED_CLIENTS_BOUND,
        CannotBlockInContext = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CANNOT_BLOCK_IN_CONTEXT,
        FlowActiveFlowIndicationUnsupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FLOW_ACTIVE_FLOW_INDICATION_UNSUPPORTED,
        UnresolvedHost = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNRESOLVED_HOST,
        CutThroughUnsupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CUT_THROUGH_UNSUPPORTED,
        CutThroughAlreadyBound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CUT_THROUGH_ALREADY_BOUND,
        CutThroughIncompatibleWithSession = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CUT_THROUGH_INCOMPATIBLE_WITH_SESSION,
        InvalidFlowOperation = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_FLOW_OPERATION,
        UnknownFlowName = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_FLOW_NAME,
        ReplicationIsStandby = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLICATION_IS_STANDBY,
        LowPriorityMsgCongestion = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LOW_PRIORITY_MSG_CONGESTION,
        LibraryNotLoaded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_LIBRARY_NOT_LOADED,
        FailedLoadingTruststore = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FAILED_LOADING_TRUSTSTORE,
        UntrustedCertificate = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNTRUSTED_CERTIFICATE,
        UntrustedCommonname = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNTRUSTED_COMMONNAME,
        CertificateDateInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CERTIFICATE_DATE_INVALID,
        FailedLoadingCertificateAndKey = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FAILED_LOADING_CERTIFICATE_AND_KEY,
        BasicAuthenticationIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_BASIC_AUTHENTICATION_IS_SHUTDOWN,
        ClientCertificateAuthenticationIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_CERTIFICATE_AUTHENTICATION_IS_SHUTDOWN,
        UntrustedClientCertificate = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNTRUSTED_CLIENT_CERTIFICATE,
        ClientCertificateDateInvalid = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_CERTIFICATE_DATE_INVALID,
        CacheRequestCancelled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CACHE_REQUEST_CANCELLED,
        DeliveryModeUnsupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DELIVERY_MODE_UNSUPPORTED,
        PublisherNotCreated = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PUBLISHER_NOT_CREATED,
        FlowUnbound = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_FLOW_UNBOUND,
        InvalidTransactedSessionId = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TRANSACTED_SESSION_ID,
        InvalidTransactionId = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_TRANSACTION_ID,
        MaxTransactedSessionsExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_TRANSACTED_SESSIONS_EXCEEDED,
        TransactedSessionNameInUse = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TRANSACTED_SESSION_NAME_IN_USE,
        ServiceUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SERVICE_UNAVAILABLE,
        NoTransactionStarted = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_TRANSACTION_STARTED,
        PublisherNotEstablished = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PUBLISHER_NOT_ESTABLISHED,
        MessagePublishFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_PUBLISH_FAILURE,
        TransactionFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TRANSACTION_FAILURE,
        MessageConsumeFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_CONSUME_FAILURE,
        EndpointModified = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_ENDPOINT_MODIFIED,
        InvalidConnectionOwner = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_INVALID_CONNECTION_OWNER,
        KerberosAuthenticationIsShutdown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_KERBEROS_AUTHENTICATION_IS_SHUTDOWN,
        CommitOrRollbackInProgress = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMMIT_OR_ROLLBACK_IN_PROGRESS,
        UnbindResponseLost = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNBIND_RESPONSE_LOST,
        MaxTransactionsExceeded = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MAX_TRANSACTIONS_EXCEEDED,
        CommitStatusUnknown = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMMIT_STATUS_UNKNOWN,
        ProxyAuthRequired = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PROXY_AUTH_REQUIRED,
        ProxyAuthFailure = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_PROXY_AUTH_FAILURE,
        NoSubscriptionMatch = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_NO_SUBSCRIPTION_MATCH,
        SubscriptionMatchError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SUBSCRIPTION_MATCH_ERROR,
        SelectorMatchError = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SELECTOR_MATCH_ERROR,
        ReplayNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_NOT_SUPPORTED,
        ReplayDisabled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_DISABLED,
        ClientInitiatedReplayNonExclusiveNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_INITIATED_REPLAY_NON_EXCLUSIVE_NOT_ALLOWED,
        ClientInitiatedReplayInactiveFlowNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_INITIATED_REPLAY_INACTIVE_FLOW_NOT_ALLOWED,
        ClientInitiatedReplayBrowserFlowNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_CLIENT_INITIATED_REPLAY_BROWSER_FLOW_NOT_ALLOWED,
        ReplayTemporaryNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_TEMPORARY_NOT_SUPPORTED,
        UnknownStartLocationType = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_UNKNOWN_START_LOCATION_TYPE,
        ReplayMessageUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_MESSAGE_UNAVAILABLE,
        ReplayStarted = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_STARTED,
        ReplayCancelled = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_CANCELLED,
        ReplayStartTimeNotAvailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_START_TIME_NOT_AVAILABLE,
        ReplayMessageRejected = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_MESSAGE_REJECTED,
        ReplayLogModified = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_LOG_MODIFIED,
        MismatchedEndpointErrorId = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MISMATCHED_ENDPOINT_ERROR_ID,
        OutOfReplayResources = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OUT_OF_REPLAY_RESOURCES,
        TopicOrSelectorModifiedOnDurableTopicEndpoint = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_TOPIC_OR_SELECTOR_MODIFIED_ON_DURABLE_TOPIC_ENDPOINT,
        ReplayFailed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_FAILED,
        CompressedSslNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_COMPRESSED_SSL_NOT_SUPPORTED,
        SharedSubscriptionsNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SHARED_SUBSCRIPTIONS_NOT_SUPPORTED,
        SharedSubscriptionsNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SHARED_SUBSCRIPTIONS_NOT_ALLOWED,
        SharedSubscriptionsEndpointNotAllowed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SHARED_SUBSCRIPTIONS_ENDPOINT_NOT_ALLOWED,
        ObjectDestroyed = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_OBJECT_DESTROYED,
        DeliveryCountNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_DELIVERY_COUNT_NOT_SUPPORTED,
        ReplayStartMessageUnavailable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_START_MESSAGE_UNAVAILABLE,
        MessageIdNotComparable = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_MESSAGE_ID_NOT_COMPARABLE,
        ReplayAnonymousNotSupported = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_REPLAY_ANONYMOUS_NOT_SUPPORTED,
        BrowsingNotSupportedOnPartitionedQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_BROWSING_NOT_SUPPORTED_ON_PARTITIONED_QUEUE,
        SelectorsNotSupportedOnPartitionedQueue = rsolace_sys::solClient_subCode_SOLCLIENT_SUBCODE_SELECTORS_NOT_SUPPORTED_ON_PARTITIONED_QUEUE,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolClientSubCodeOrRaw {
    SubCode(SolClientSubCode),
    Raw(rsolace_sys::solClient_subCode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorInfo {
    pub sub_code: SolClientSubCodeOrRaw,
    pub error_str: String,
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ErrorInfo {{ sub_code: {:?}, error_str: {} }}",
            self.sub_code, self.error_str
        )
    }
}

impl ErrorInfo {
    pub fn from_error_info_ptr(
        error_info_ptr: *const rsolace_sys::solClient_errorInfo,
    ) -> Option<Self> {
        if error_info_ptr.is_null() {
            None
        } else {
            unsafe {
                let error_info = &*error_info_ptr;
                Some(ErrorInfo {
                    sub_code: match SolClientSubCode::from_u32(
                        error_info.subCode.try_into().unwrap(),
                    ) {
                        Some(sub_code) => SolClientSubCodeOrRaw::SubCode(sub_code),
                        None => SolClientSubCodeOrRaw::Raw(error_info.subCode),
                    },
                    error_str: std::ffi::CStr::from_ptr(error_info.errorStr.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                })
            }
        }
    }
}
