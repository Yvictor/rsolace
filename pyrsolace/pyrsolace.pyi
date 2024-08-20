from typing import Callable, Optional, List
from enum import Enum

class LogLevel(Enum):
    Debug = 1
    Info = 2
    Warn = 3
    Error = 4

class ReturnCode(Enum):
    Ok = 0
    WouldBlock = 1
    InProgress = 2
    NotReady = 3
    Eos = 4
    NotFound = 5
    NoEvent = 6
    InComplete = 7
    Rollback = 8
    Fail = 9

class DeliveryMode(Enum):
    Direct = 0
    Persistent = 16
    NonPersistent = 32

class DestType(Enum):
    Topic = 0
    Queue = 1
    TopicTemp = 2
    QueueTemp = 3

class SessionEvent(Enum):
    UpNotice = 0
    DownError = 1
    ConnectFailedError = 2

class SubscribeFlag(Enum):
    WaitForConfirm = 2
    LocalDispatchOnly = 8
    RequestConfirm = 16

class CacheRequestFlag(Enum):
    NoSubscribe = 1
    LiveDataFulfill = 2
    LiveDataQueue = 4
    LiveDataFlowThru = 8
    NowaitReply = 16

class CacheStatus(Enum):
    Invalid = -1
    Live = 0
    Cache = 1
    Suspect = 2

def init_tracing_logger(
    level: LogLevel,
    display_line_number: bool = False,
    display_thread_names: bool = False,
    display_thread_ids: bool = False,
    display_filename: bool = False,
) -> None: ...

class Event:
    info: str
    response_code: int
    session_event: SessionEvent
    def get_session_event_string(self) -> str: ...

class Dest:
    def __init__(self, dest_type: DestType, dest: str) -> Dest: ...
    def get_dest_type(self) -> DestType: ...
    def set_dest_type(self, dest_type: DestType) -> None: ...
    def get_dest_name(self) -> str: ...
    def set_dest_name(self, dest_name: str) -> None: ...

class Msg:
    delivery_mode: DeliveryMode
    dest: Optional[Dest]
    topic: Optional[str]
    data: bytes
    reply_to: Optional[Dest]
    reply_topic: Optional[str]
    corr_id: Optional[str]
    is_reply: bool
    eligible: bool
    is_p2p: bool
    cos: int
    is_delivery_to_one: bool
    is_discard_indication: bool
    is_cache: bool
    cache_status: CacheStatus
    cache_request_id: Optional[int]
    sender_id: Optional[str]
    sender_timestamp: Optional[int]
    recv_timestamp: Optional[int]
    seq: Optional[int]
    msg_type: Optional[str]

    def __init__(
        self,
        topic: Optional[str],
        data: Optional[bytes],
        corr_id: Optional[str],
        reply_topic: Optional[str],
        is_reply: Optional[bool] = None,
        eligible: Optional[bool] = None,
        cos: int = 1,
        is_delivery_to_one: Optional[bool] = True,
    ) -> Msg: ...
    def set_user_prop(self, key: str, value: str, map_size: int) -> None: ...
    def get_user_prop(self, key: str) -> str: ...
    def dump(self) -> str: ...

class MsgReceiver:
    def recv(self) -> Msg: ...

class Client:
    def __init__(self) -> Client: ...
    def set_msg_callback(self, callback: Callable[[Msg], None]) -> None: ...
    def set_event_callback(self, callback: Callable[[Event], None]) -> None: ...
    def set_request_callback(self, callback: Callable[[Msg], None]) -> None: ...
    def set_p2p_callback(self, callback: Callable[[Msg], None]) -> None: ...
    def connect(
        self,
        host: str,
        vpn: str,
        username: str,
        password: str,
        client_name: Optional[str],
        connect_timeout_ms: int = 3000,
        reconnect_retries: int = 10,
        keep_alive_ms: int = 3000,
        reconnect_retry_wait: int = 3000,
        keep_alive_limit: int = 3,
        compression_level: int = 1,
        connect_retries: int = 3,
        reapply_subscriptions: bool = True,
        generate_sender_id: bool = False,
        generate_sequence_number: bool = False,
        generate_send_timestamps: bool = False,
        generate_rcv_timestamps: bool = False,
    ) -> bool: ...
    def disconnect(self) -> None: ...
    def subscribe(self, topic: str) -> ReturnCode: ...
    def unsubscribe(self, topic: str) -> ReturnCode: ...
    def subscribe_ext(self, topic: str, flag: SubscribeFlag) -> ReturnCode: ...
    def unsubscribe_ext(self, topic: str, flag: SubscribeFlag) -> ReturnCode: ...
    def send_msg(self, msg: Msg) -> ReturnCode: ...
    def send_multiple_msg(self, msgs: List[Msg]) -> ReturnCode: ...
    def send_cache_request(
        self,
        topic: str,
        request_id: int,
        cache_name: str,
        max_msgs: int = 0,
        max_age: int = 0,
        request_reply_timeout: int = 10000,
        flag: CacheRequestFlag = CacheRequestFlag.LiveDataFlowThru,
    ) -> ReturnCode: ...
    def send_request(self, msg: Msg, timeout: int = 5000) -> MsgReceiver: ...
    def send_reply(self, rx_msg: Msg, reply_msg: Msg) -> ReturnCode: ...
    def modify_client_info(
        self, app_description: Optional[str], client_name: Optional[str]
    ) -> ReturnCode: ...
