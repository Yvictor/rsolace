# rsolace

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/rsolace.svg)](https://crates.io/crates/rsolace)

High-performance Rust bindings for the Solace messaging system with **native async/await support**.

## ✨ Features

- 🚀 **Full Async Support**: Native async/await with kanal + tokio
- ⚡ **High Performance**: Zero-copy message handling, efficient channels
- 🔄 **Flexible Patterns**: Sync, async, or mixed messaging patterns
- 📦 **Complete API**: Pub/Sub, Request/Reply, Message Caching, Events
- 🛡️ **Type Safe**: Comprehensive error handling with snafu
- 🔧 **Easy Integration**: Drop-in replacement for existing Solace apps

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
rsolace = { version = "0.2.0", features = ["tokio", "channel"] }
tokio = { version = "1.28", features = ["full"] }
```

### Async Example

```rust
use rsolace::solclient::SolClient;
use rsolace::SessionProps;
use rsolace::solmsg::SolMsg;
use rsolace::types::SolClientLogLevel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with async support
    let mut client = SolClient::new(SolClientLogLevel::Notice)?;
    
    // Get async receivers
    let msg_receiver = client.get_async_msg_receiver();
    let event_receiver = client.get_async_event_receiver();
    
    // Handle messages asynchronously
    tokio::spawn(async move {
        while let Ok(msg) = msg_receiver.recv().await {
            println!("Async received: {}", msg.get_topic().unwrap_or_default());
        }
    });
    
    // Handle events asynchronously
    tokio::spawn(async move {
        while let Ok(event) = event_receiver.recv().await {
            println!("Event: {:?}", event);
        }
    });
    
    // Connect
    let props = SessionProps::default()
        .host("tcp://localhost:55555")
        .vpn("default")
        .username("admin")
        .password("admin");
    
    client.connect(props);
    client.subscribe("test/topic/*");
    
    // Send async request
    let mut msg = SolMsg::new()?;
    msg.set_topic("request/topic");
    msg.set_binary_attachment(b"Hello async world!");
    
    let response = client.send_request_async(&msg).await?;
    println!("Response: {:?}", response.get_binary_attachment());
    
    Ok(())
}
```

### Sync Example

```rust
use rsolace::solclient::SolClient;
use rsolace::SessionProps;
use rsolace::types::SolClientLogLevel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SolClient::new(SolClientLogLevel::Notice)?;
    
    // Connect
    let props = SessionProps::default()
        .host("tcp://localhost:55555")
        .vpn("default")
        .username("admin")
        .password("admin");
    
    client.connect(props);
    client.subscribe("events/*");
    
    // Handle messages synchronously
    let receiver = client.get_msg_receiver();
    std::thread::spawn(move || {
        while let Ok(msg) = receiver.recv() {
            println!("Received: {}", msg.get_topic().unwrap_or_default());
        }
    });
    
    // Send message
    let mut msg = SolMsg::new()?;
    msg.set_topic("events/test");
    msg.set_binary_attachment(b"Hello world!");
    client.send_msg(&msg);
    
    Ok(())
}
```

## 🔄 Migration from crossbeam-channel

We've migrated from `crossbeam-channel` to [`kanal`](https://github.com/fereidani/kanal) for native async support:

```rust
// Before (crossbeam-channel)
let receiver = client.get_msg_receiver();
receiver.recv().unwrap(); // Sync only

// After (kanal) - Both sync and async!
let receiver = client.get_msg_receiver();
receiver.recv().unwrap();                    // Sync (unchanged)

let async_receiver = client.get_async_msg_receiver();
async_receiver.recv().await.unwrap();        // Async (NEW!)
```

## 🛠️ Features

Enable features in your `Cargo.toml`:

```toml
[dependencies]
rsolace = { version = "0.2.0", features = ["tokio", "channel", "raw"] }
```

| Feature | Description |
|---------|-------------|
| `channel` | Kanal-based sync/async channel support (default) |
| `tokio` | Async support with tokio runtime |
| `raw` | Direct callback-based message handling |

## 📋 API Overview

### Client Management
```rust
let mut client = SolClient::new(SolClientLogLevel::Notice)?;
client.connect(session_props);
client.disconnect();
```

### Messaging Patterns
```rust
// Pub/Sub
client.subscribe("topic/*");
client.send_msg(&msg);

// Request/Reply (Sync)
let response = client.send_request(&request, timeout)?;

// Request/Reply (Async)
let response = client.send_request_async(&request).await?;
```

### Message Handling
```rust
// Sync receivers
let msg_receiver = client.get_msg_receiver();
let event_receiver = client.get_event_receiver();
let request_receiver = client.get_request_receiver();
let p2p_receiver = client.get_p2p_receiver();

// Async receivers (with tokio feature)
let async_msg_receiver = client.get_async_msg_receiver();
let async_event_receiver = client.get_async_event_receiver();
let async_request_receiver = client.get_async_request_receiver();
let async_p2p_receiver = client.get_async_p2p_receiver();
```

### Message Creation
```rust
let mut msg = SolMsg::new()?;
msg.set_topic("my/topic");
msg.set_binary_attachment(b"payload");
msg.set_delivery_mode(SolClientDeliveryMode::Persistent);
msg.set_correlation_id("req-123");
```

## 🔧 Configuration

### Session Properties
```rust
let props = SessionProps::default()
    .host("tcp://broker:55555")
    .vpn("vpn_name")
    .username("user")
    .password("pass")
    .compression_level(5)           // 1-9
    .connect_retries(3)
    .reconnect_retries(10)
    .keep_alive_int_ms(3000)
    .reapply_subscriptions(true)
    .generate_sender_id(true)
    .generate_timestamps(true);
```

## 🏗️ Building

```bash
# Basic build
cargo build --release

# With async support
cargo build --features "tokio,channel" --release

# All features
cargo build --features "tokio,channel,raw" --release

# Run examples
cargo run --example pubsub
cargo run --example async_example --features tokio
```

## 📖 Examples

See the [`examples/`](examples/) directory for comprehensive usage patterns:

- [`pubsub.rs`](examples/pubsub.rs) - Publisher/Subscriber pattern
- [`requester.rs`](examples/requester.rs) - Request/Reply client
- [`replier.rs`](examples/replier.rs) - Request/Reply server
- [`cache.rs`](examples/cache.rs) - Message caching
- [`async_example.rs`](examples/async_example.rs) - Async patterns

## 🧪 Testing

```bash
# Run tests
cargo test

# Test with async features
cargo test --features tokio
```

## 📚 Documentation

- **Main Project**: See [root README](../README.md) for full documentation
- **Python Bindings**: Check [pyrsolace](../pyrsolace/README.md)
- **API Docs**: Run `cargo doc --open`
- **Examples**: Comprehensive examples in `examples/` directory

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass: `cargo test --features tokio`
5. Submit a pull request

## 📄 License

GPL-3.0-only License - see [LICENSE](../LICENSE) for details.

---

**Need Python bindings?** Check out [pyrsolace](../pyrsolace/) for full async Python support! 🐍