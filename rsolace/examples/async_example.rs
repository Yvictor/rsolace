use dotenvy::dotenv;
use rsolace::solclient::SolClient;
use rsolace::SessionProps;
use rsolace::types::{SolClientLogLevel, SolClientSubscribeFlags};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut solclient = SolClient::new(SolClientLogLevel::Notice)?;

    // Get async receivers for handling messages asynchronously
    let msg_recv_async = solclient.get_async_msg_receiver();
    let event_recv_async = solclient.get_async_event_receiver();

    // Spawn async task to handle messages
    let msg_handle = tokio::spawn(async move {
        while let Ok(msg) = msg_recv_async.recv().await {
            tracing::info!(
                "Async received message: {} {:?}",
                msg.get_topic().unwrap_or_default(),
                msg.get_binary_attachment().unwrap()
            );
        }
    });

    // Spawn async task to handle events
    let event_handle = tokio::spawn(async move {
        while let Ok(event) = event_recv_async.recv().await {
            tracing::info!("Async received event: {:?}", event);
        }
    });

    // Configure session properties
    let props = SessionProps::default()
        .host(&std::env::var("SOLACE_HOST").unwrap_or("218.32.76.102:80".to_string()))
        .vpn(&std::env::var("SOLACE_VPN").unwrap_or("ITIC_UAT".to_string()))
        .username(&std::env::var("SOLACE_USERNAME").unwrap_or("".to_string()))
        .password(&std::env::var("SOLACE_PASSWORD").unwrap_or("".to_string()))
        .compression_level(5);

    // Connect to Solace
    let connected = solclient.connect(props);
    if connected {
        tracing::info!("Connected to Solace successfully!");

        // Subscribe to a topic
        let result = solclient.subscribe_ext(
            "TIC/v1/FOP/*/TFE/TXFG5",
            SolClientSubscribeFlags::RequestConfirm,
        );
        tracing::info!("Subscription result: {:?}", result);

        // Wait a bit to receive messages
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    } else {
        tracing::error!("Failed to connect to Solace");
    }

    // Clean up
    solclient.disconnect();

    // Cancel async tasks
    msg_handle.abort();
    event_handle.abort();

    Ok(())
}
