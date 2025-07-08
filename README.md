# rsolace

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Python](https://img.shields.io/badge/python-3.7+-blue.svg)](https://www.python.org)

High-performance Rust bindings for the Solace messaging system with full **async/await support** for both Rust and Python.

## ✨ Key Features

- 🚀 **Full Async Support**: Native async/await patterns with kanal + tokio (Rust) and asyncio (Python)
- 🔄 **Sync + Async Flexibility**: Choose the best pattern for your use case
- 🐍 **Python Bindings**: Complete Python API with proper GIL release
- 📦 **Enterprise Messaging**: Pub/Sub, Request/Reply, Message Caching, Event Handling
- ⚡ **High Performance**: Zero-copy message handling, efficient channel-based architecture
- 🛡️ **Type Safe**: Comprehensive error handling and type safety
- 🔧 **Easy Integration**: Drop-in replacement for existing Solace applications

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   pyrsolace     │    │     rsolace      │    │  rsolace-sys    │
│                 │    │                  │    │                 │
│ Python Bindings │◄───┤ High-level Rust  │◄───┤ Low-level FFI   │
│ + Async Support │    │ + Async Channels │    │ Bindings        │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## 🚀 Quick Start

### Rust

```toml
# Cargo.toml
[dependencies]
rsolace = { version = "0.2.0", features = ["tokio", "channel"] }
tokio = { version = "1.28", features = ["full"] }
```

```rust
use rsolace::solclient::{SessionProps, SolClient};
use rsolace::types::SolClientLogLevel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let mut client = SolClient::new(SolClientLogLevel::Notice)?;
    
    // Get async receivers
    let msg_receiver = client.get_async_msg_receiver();
    
    // Handle messages asynchronously
    tokio::spawn(async move {
        while let Ok(msg) = msg_receiver.recv().await {
            println!("Received: {}", msg.get_topic().unwrap_or_default());
        }
    });
    
    // Connect
    let props = SessionProps::default()
        .host("your-solace-host:55555")
        .vpn("your-vpn")
        .username("username")
        .password("password");
    
    client.connect(props);
    client.subscribe("test/topic/*");
    
    // Send async request
    let mut msg = SolMsg::new()?;
    msg.set_topic("request/topic");
    msg.set_binary_attachment(b"Hello World!");
    
    let response = client.send_request_async(&msg).await?;
    println!("Response: {:?}", response.get_binary_attachment());
    
    Ok(())
}
```

### Python

```bash
# Install
pip install pyrsolace
# or with uv
uv add pyrsolace
```

```python
import asyncio
import pyrsolace

async def main():
    client = pyrsolace.Client()
    
    # Connect
    client.connect(
        host="your-solace-host:55555",
        vpn="your-vpn",
        username="username",
        password="password"
    )
    
    # Subscribe
    client.subscribe("test/topic/*")
    
    # Get async receiver
    async_receiver = client.get_async_msg_receiver()
    
    # Handle messages asynchronously
    async def message_handler():
        while True:
            msg = await async_receiver.recv()
            print(f"Received: {msg.topic} - {msg.data}")
    
    # Start handler
    asyncio.create_task(message_handler())
    
    # Send messages
    for i in range(5):
        msg = pyrsolace.Msg(
            topic="test/topic/async",
            data=f"Message {i}".encode()
        )
        client.send_msg(msg)
        await asyncio.sleep(1)
    
    # Send async request
    request = pyrsolace.Msg(topic="request/topic", data=b"Hello!")
    response = await client.send_request_async(request)
    print(f"Response: {response.data}")

if __name__ == "__main__":
    asyncio.run(main())
```

## 🔄 Sync vs Async

### Choose Your Pattern

| Pattern | Best For | Rust | Python |
|---------|----------|------|--------|
| **Sync** | Simple apps, blocking I/O | `receiver.recv()` | `receiver.recv()` |
| **Async** | High concurrency, non-blocking | `receiver.recv().await` | `await receiver.recv()` |
| **Mixed** | Gradual migration | Both in same app | Both in same app |

### Migration Guide

**From crossbeam-channel to kanal (Rust)**:
```rust
// Before
let (s, r) = crossbeam_channel::unbounded();
r.recv().unwrap(); // Sync only

// After  
let (s, r) = kanal::unbounded();
r.recv().unwrap();                    // Sync (unchanged)
r.as_async().recv().await.unwrap();   // Async (NEW!)
```

**Python GIL Enhancement**:
```python
# Before: Blocked other threads
msg = receiver.recv()

# After: Properly releases GIL
msg = receiver.recv()  # Other threads can run

# NEW: True async
msg = await async_receiver.recv()
```

## 📋 Feature Matrix

| Feature | Rust Sync | Rust Async | Python Sync | Python Async |
|---------|-----------|------------|-------------|--------------|
| Pub/Sub | ✅ | ✅ | ✅ | ✅ |
| Request/Reply | ✅ | ✅ | ✅ | ✅ |
| Message Caching | ✅ | ✅ | ✅ | ✅ |
| Event Handling | ✅ | ✅ | ✅ | ✅ |
| GIL Release | N/A | N/A | ✅ | ✅ |
| Backpressure | ✅ | ✅ | ✅ | ✅ |
| Zero-copy | ✅ | ✅ | ✅ | ✅ |

## 🛠️ Building

### Prerequisites

1. **Solace Client Library**: Download `solclient-7.25.0.10.tar.gz`
2. **Rust**: 1.70+ with `cargo`
3. **Python**: 3.7+ (for Python bindings)

### Rust Library

```bash
# Sync support only
cargo build --release

# Full async support  
cargo build --features "tokio,channel" --release

# All features
cargo build --features "tokio,channel,raw" --release

# Run examples
cargo run --example pubsub
cargo run --example async_example --features tokio
```

### Python Bindings

```bash
cd pyrsolace/

# Using uv (recommended)
uv build
uv pip install -e .

# Using maturin
pip install maturin
maturin develop --release
```

## 📖 Examples

### Basic Patterns

**Publisher**:
```rust
let mut msg = SolMsg::new()?;
msg.set_topic("events/user/login");
msg.set_binary_attachment(b"user123");
client.send_msg(&msg);
```

**Subscriber**:
```rust
client.subscribe("events/user/*");
let receiver = client.get_msg_receiver();
while let Ok(msg) = receiver.recv() {
    println!("Event: {}", msg.get_topic()?);
}
```

**Request/Reply**:
```rust
// Sync
let response = client.send_request(&request_msg, 5000)?;

// Async  
let response = client.send_request_async(&request_msg).await?;
```

### Advanced Patterns

**Async Message Processing**:
```rust
let receiver = client.get_async_msg_receiver();
tokio::spawn(async move {
    while let Ok(msg) = receiver.recv().await {
        tokio::spawn(async move {
            process_message(msg).await;
        });
    }
});
```

**Python Async with Backpressure**:
```python
import asyncio

async def bounded_processor(client):
    semaphore = asyncio.Semaphore(10)  # Max 10 concurrent
    receiver = client.get_async_msg_receiver()
    
    while True:
        msg = await receiver.recv()
        asyncio.create_task(process_with_limit(msg, semaphore))

async def process_with_limit(msg, semaphore):
    async with semaphore:
        await process_message(msg)
```

## 🔧 Configuration

### Connection Properties

```rust
let props = SessionProps::default()
    .host("tcp://localhost:55555")
    .vpn("default")
    .username("admin")
    .password("admin")
    .compression_level(5)              // 1-9
    .connect_retries(3)
    .reconnect_retries(10)
    .keep_alive_int_ms(3000)
    .reapply_subscriptions(true);
```

### Message Properties

```rust
let mut msg = SolMsg::new()?;
msg.set_topic("my/topic");
msg.set_delivery_mode(SolClientDeliveryMode::Persistent);
msg.set_correlation_id("req-123");
msg.set_user_prop("priority", "high", 10);
msg.set_binary_attachment(b"payload");
```

## 🎯 Use Cases

### Event-Driven Microservices
```rust
// Service A publishes events
msg.set_topic("order/created");
client.send_msg(&msg);

// Service B subscribes to events  
client.subscribe("order/*");
let receiver = client.get_async_msg_receiver();
```

### Real-time Data Streaming
```python
async def stream_processor():
    receiver = client.get_async_msg_receiver()
    async for msg in message_stream(receiver):
        await process_real_time_data(msg)
```

### Request/Reply Services
```rust
// Server
let request_receiver = client.get_async_request_receiver();
while let Ok(request) = request_receiver.recv().await {
    let response = process_request(request).await;
    client.send_reply(&request, &response);
}
```

## 🔍 Monitoring & Debugging

### Logging
```rust
// Rust
tracing_subscriber::fmt::init();

// Python  
pyrsolace.init_tracing_logger(
    level=pyrsolace.LogLevel.Debug,
    display_line_number=True
)
```

### Message Inspection
```rust
println!("{}", msg.dump()); // Pretty print message
println!("Topic: {}", msg.get_topic()?);
println!("Size: {} bytes", msg.get_binary_attachment()?.len());
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality  
4. Ensure all tests pass: `cargo test --features tokio`
5. Submit a pull request

## 📚 Documentation

- **Examples**: See `rsolace/examples/` for comprehensive usage patterns
- **API Reference**: Check `pyrsolace/pyrsolace.pyi` for Python type hints
- **Async Patterns**: Refer to this README and `CLAUDE.md` for detailed async examples
- **Solace Docs**: [Official Solace Documentation](https://docs.solace.com/)

## 📄 License

This project is licensed under the GPL-3.0-only License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on the [Solace PubSub+ C API](https://solace.com/)
- Powered by [kanal](https://github.com/fereidani/kanal) for async channel support
- Python bindings via [PyO3](https://pyo3.rs/)
- Async Python support through [pyo3-asyncio](https://github.com/awestlake87/pyo3-asyncio)

---

**Ready to build high-performance, async messaging applications?** 🚀

```bash
# Get started now
git clone https://github.com/Yvictor/rsolace.git
cd rsolace
cargo run --example async_example --features tokio
```
