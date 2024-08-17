from typing import Callable, Optional
from enum import Enum

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
    sender_timestamp: Optional[int]

    def __init__(
        self,
        topic: Optional[str],
        data: Optional[bytes],
        reply_topic: Optional[str],
        is_reply: Optional[bool] = None,
        eligible: Optional[bool] = None,
        cos: int = 1,
        is_delivery_to_one: Optional[bool] = True,
    ) -> Msg: ...
    def set_user_prop(self, key: str, value: str, map_size: int) -> None: ...
    def get_user_prop(self, key: str) -> str: ...
    def dump(self) -> str: ...

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
    def publish(self, topic: str, payload: bytes) -> None: ...
    def subscribe(self, topic: str) -> ReturnCode: ...
    def unsubscribe(self, topic: str) -> ReturnCode: ...
    def send_msg(self, msg: Msg) -> ReturnCode: ...
    # def send_cache_request(self,  topic: &str, request_id: u64, cache_name: &str, max_msgs: u32, max_age: u32, request_reply_timeout: u32, flag: CacheRequestFlag) -> ReturnCode: ...
    # def send_request(self, msg: Msg, timeout: int = 3000) -> ReturnCode: ...
    def send_reply(self, rx_msg: Msg, reply_msg: Msg) -> ReturnCode: ...
    def modify_client_info(
        self, app_description: Optional[str], client_name: Optional[str]
    ) -> ReturnCode: ...
