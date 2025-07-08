# pyrsolace

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Python](https://img.shields.io/badge/python-3.7+-blue.svg)](https://www.python.org)
[![PyPI](https://img.shields.io/pypi/v/pyrsolace.svg)](https://pypi.org/project/pyrsolace/)

Python bindings for rsolace with **full async/await support** and proper GIL release.

## ✨ Key Features

- 🐍 **Full asyncio Support**: Native async/await patterns with asyncio
- 🔓 **GIL Release**: Properly releases GIL during blocking operations
- 🔄 **Sync + Async**: Choose the best pattern for your use case
- 📦 **Complete API**: Pub/Sub, Request/Reply, Message Caching, Events
- ⚡ **High Performance**: Zero-copy message handling from Rust
- 🛡️ **Type Safe**: Complete type hints with `.pyi` files

## 🚀 Installation

```bash
# Using pip
pip install pyrsolace

# Using uv (recommended)
uv add pyrsolace
```

## 🔥 Quick Start

### Async Example (NEW!)

```python
import asyncio
import pyrsolace

async def main():
    # Initialize client
    client = pyrsolace.Client()
    
    # Connect to Solace broker
    connected = client.connect(
        host="tcp://localhost:55555",
        vpn="default",
        username="admin",
        password="admin",
        compression_level=5
    )
    
    if not connected:
        print("Failed to connect")
        return
    
    # Subscribe to topics
    client.subscribe_ext("test/topic/*", pyrsolace.SubscribeFlag.RequestConfirm)
    
    # Get async receivers
    async_msg_receiver = client.get_async_msg_receiver()
    async_event_receiver = client.get_async_event_receiver()
    
    # Handle messages asynchronously
    async def message_handler():
        while True:
            try:
                msg = await async_msg_receiver.recv()
                print(f"Async received: {msg.topic} - {msg.data}")
            except Exception as e:
                print(f"Message handler error: {e}")
                break
    
    # Handle events asynchronously
    async def event_handler():
        while True:
            try:
                event = await async_event_receiver.recv()
                print(f"Event: {event.session_event}")
            except Exception as e:
                print(f"Event handler error: {e}")
                break
    
    # Start async handlers
    msg_task = asyncio.create_task(message_handler())
    event_task = asyncio.create_task(event_handler())
    
    # Send some test messages
    for i in range(5):
        msg = pyrsolace.Msg(
            topic="test/topic/async",
            data=f"Async message {i}".encode()
        )
        client.send_msg(msg)
        await asyncio.sleep(1)
    
    # Send async request (true async, no blocking!)
    try:
        request_msg = pyrsolace.Msg(
            topic="test/request", 
            data=b"Hello async world!", 
            corr_id="req123"
        )
        
        response = await client.send_request_async(request_msg)
        print(f"Async response: {response.data}")
    except Exception as e:
        print(f"Async request failed: {e}")
    
    # Cleanup
    await asyncio.sleep(2)
    msg_task.cancel()
    event_task.cancel()
    client.disconnect()

if __name__ == "__main__":
    asyncio.run(main())
```

### Sync Example (Enhanced with GIL Release)

```python
import pyrsolace
import threading
import time

def message_handler(receiver, name):
    """Handle messages synchronously with proper GIL release."""
    while True:
        try:
            # This properly releases GIL, allowing other threads to run
            msg = receiver.recv()
            print(f"{name} received: {msg.topic} - {msg.data}")
        except Exception as e:
            print(f"{name} handler error: {e}")
            break

def main():
    client = pyrsolace.Client()
    
    # Connect
    connected = client.connect(
        host="tcp://localhost:55555",
        vpn="default",
        username="admin", 
        password="admin"
    )
    
    if not connected:
        print("Failed to connect")
        return
    
    # Subscribe
    client.subscribe("test/topic/*")
    
    # Get receivers
    msg_receiver = client.get_msg_receiver()
    event_receiver = client.get_event_receiver()
    
    # Start background threads (GIL is properly released)
    msg_thread = threading.Thread(
        target=message_handler, 
        args=(msg_receiver, "Messages")
    )
    event_thread = threading.Thread(
        target=message_handler, 
        args=(event_receiver, "Events")
    )
    
    msg_thread.start()
    event_thread.start()
    
    # Send messages
    for i in range(5):
        msg = pyrsolace.Msg(
            topic="test/topic/sync",
            data=f"Sync message {i}".encode()
        )
        client.send_msg(msg)
        time.sleep(1)
    
    client.disconnect()

if __name__ == "__main__":
    main()
```

### Callback-based Example

```python
import pyrsolace
import time

def on_message(msg):
    """Message callback function."""
    print(f"Callback received: {msg.topic} - {msg.data}")

def on_event(event):
    """Event callback function."""
    print(f"Event: {event.session_event} - {event.info}")

def main():
    client = pyrsolace.Client()
    
    # Set callbacks
    client.set_msg_callback(on_message)
    client.set_event_callback(on_event)
    
    # Connect and subscribe
    client.connect(
        host="tcp://localhost:55555",
        vpn="default",
        username="admin",
        password="admin"
    )
    
    client.subscribe("test/topic/*")
    
    # Send messages
    for i in range(5):
        msg = pyrsolace.Msg(
            topic="test/topic/callback",
            data=f"Callback message {i}".encode()
        )
        client.send_msg(msg)
        time.sleep(1)
    
    client.disconnect()

if __name__ == "__main__":
    main()
```

## 🔄 Sync vs Async

| Pattern | Best For | Usage | GIL Behavior |
|---------|----------|-------|--------------|
| **Callbacks** | Simple event handling | `client.set_msg_callback(fn)` | Released during callback |
| **Sync Receivers** | Threading, blocking I/O | `receiver.recv()` | Released during recv |
| **Async Receivers** | High concurrency | `await async_receiver.recv()` | N/A (async) |

### Migration from Sync to Async

```python
# Before: Sync only
receiver = client.get_msg_receiver()
msg = receiver.recv()  # Blocks thread (but releases GIL)

# After: True async
async_receiver = client.get_async_msg_receiver()
msg = await async_receiver.recv()  # Non-blocking, async

# Mixed: Use both in same application
sync_receiver = client.get_msg_receiver()     # For background threads
async_receiver = client.get_async_msg_receiver()  # For async tasks
```

## 📋 API Reference

### Client Class

```python
class Client:
    def connect(self, host: str, vpn: str, username: str, password: str, ...) -> bool
    def disconnect(self) -> None
    def subscribe(self, topic: str) -> ReturnCode
    def subscribe_ext(self, topic: str, flag: SubscribeFlag) -> ReturnCode
    
    # Message sending
    def send_msg(self, msg: Msg) -> ReturnCode
    def send_reply(self, rx_msg: Msg, reply_msg: Msg) -> ReturnCode
    
    # Sync receivers (with GIL release)
    def get_msg_receiver(self) -> MsgReceiver
    def get_request_receiver(self) -> MsgReceiver
    def get_p2p_receiver(self) -> MsgReceiver
    def get_event_receiver(self) -> EventReceiver
    
    # Async receivers (NEW!)
    def get_async_msg_receiver(self) -> AsyncMsgReceiver
    def get_async_request_receiver(self) -> AsyncMsgReceiver
    def get_async_p2p_receiver(self) -> AsyncMsgReceiver
    def get_async_event_receiver(self) -> AsyncEventReceiver
    
    # Request/Reply
    def send_request(self, msg: Msg, timeout: int) -> MsgReceiver
    async def send_request_async(self, msg: Msg) -> Msg  # NEW!
    
    # Callbacks
    def set_msg_callback(self, callback: Callable[[Msg], None]) -> None
    def set_event_callback(self, callback: Callable[[Event], None]) -> None
```

### Message Class

```python
class Msg:
    def __init__(self, topic: str = None, data: bytes = None, ...) -> None
    
    # Properties
    topic: str
    data: bytes
    corr_id: str
    reply_topic: str
    delivery_mode: DeliveryMode
    
    # Methods
    def set_user_prop(self, key: str, value: str) -> None
    def get_user_prop(self, key: str) -> str
    def dump(self) -> str
```

### Receiver Classes

```python
class MsgReceiver:
    def recv(self) -> Msg  # Releases GIL

class AsyncMsgReceiver:
    async def recv(self) -> Msg  # True async

class EventReceiver:
    def recv(self) -> Event  # Releases GIL

class AsyncEventReceiver:
    async def recv(self) -> Event  # True async
```

## 🛠️ Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/Yvictor/rsolace.git
cd rsolace/pyrsolace

# Using uv (recommended)
uv build
uv pip install -e .

# Using maturin
pip install maturin
maturin develop --release
```

### Running Tests

```bash
# Run tests
uv run pytest tests/

# Run specific tests
uv run pytest tests/test_msg.py -v
```

## 🔧 Configuration

### Connection Parameters

```python
client.connect(
    host="tcp://broker:55555",          # Broker URL
    vpn="vpn_name",                     # VPN name
    username="user",                    # Username
    password="pass",                    # Password
    client_name="my_client",            # Client identifier
    compression_level=5,                # 1-9 (higher = more compression)
    connect_timeout_ms=30000,           # Connection timeout
    connect_retries=3,                  # Retry attempts
    reconnect_retries=10,               # Auto-reconnect attempts
    keep_alive_ms=3000,                 # Keep-alive interval
    reapply_subscriptions=True,         # Restore subs on reconnect
    generate_sender_id=True,            # Add sender ID to messages
    generate_timestamps=True,           # Add timestamps
)
```

### Message Properties

```python
msg = pyrsolace.Msg(
    topic="my/topic",
    data=b"payload",
    corr_id="request-123",
    reply_topic="reply/topic",
    delivery_mode=pyrsolace.DeliveryMode.Persistent
)

# User properties
msg.set_user_prop("priority", "high")
msg.set_user_prop("version", "1.0")
```

## 🎯 Advanced Examples

### Async Producer-Consumer Pattern

```python
import asyncio
from asyncio import Queue

async def producer(client, queue):
    """Produce messages to queue."""
    for i in range(100):
        msg = pyrsolace.Msg(
            topic=f"data/stream/{i % 10}",
            data=f"Data packet {i}".encode()
        )
        await queue.put(msg)
        await asyncio.sleep(0.1)

async def consumer(client, queue):
    """Consume messages from queue."""
    while True:
        msg = await queue.get()
        client.send_msg(msg)
        queue.task_done()

async def message_processor(client):
    """Process incoming messages."""
    receiver = client.get_async_msg_receiver()
    while True:
        msg = await receiver.recv()
        # Process message asynchronously
        await process_message(msg)

async def main():
    client = pyrsolace.Client()
    client.connect(...)
    
    queue = Queue(maxsize=100)
    
    # Start producer, consumer, and processor
    await asyncio.gather(
        producer(client, queue),
        consumer(client, queue),
        message_processor(client)
    )
```

### Request/Reply Service

```python
async def request_handler(client):
    """Handle incoming requests asynchronously."""
    receiver = client.get_async_request_receiver()
    
    while True:
        request = await receiver.recv()
        
        # Process request
        response_data = await process_request(request.data)
        
        # Send reply
        reply = pyrsolace.Msg(
            topic=request.reply_topic,
            data=response_data,
            corr_id=request.corr_id
        )
        client.send_reply(request, reply)
```

## 🚀 Performance Tips

### Async Best Practices

1. **Use Semaphores**: Limit concurrent operations
```python
semaphore = asyncio.Semaphore(10)
async with semaphore:
    await process_message(msg)
```

2. **Batch Operations**: Group related operations
```python
messages = []
async for msg in message_stream():
    messages.append(msg)
    if len(messages) >= 100:
        await process_batch(messages)
        messages.clear()
```

3. **Graceful Shutdown**: Cancel tasks properly
```python
try:
    await main_task
except asyncio.CancelledError:
    await cleanup()
```

## 📚 Documentation

- **Main Project**: See [root README](../README.md) for complete documentation
- **Rust Library**: Check [rsolace](../rsolace/README.md) for Rust-specific features
- **Type Hints**: Complete API in [`pyrsolace.pyi`](pyrsolace.pyi)
- **Examples**: More examples in [`tests/`](tests/) directory

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Run tests: `uv run pytest`
5. Submit a pull request

## 📄 License

GPL-3.0-only License - see [LICENSE](../LICENSE) for details.

---

**Powered by rsolace** ⚡ - High-performance Rust Solace bindings
