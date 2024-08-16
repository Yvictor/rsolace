use dotenvy::dotenv;
use rsolace::solcache::CacheSessionProps;
use rsolace::solclient::{SessionProps, SolClient};
use rsolace::types::SolClientLogLevel;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let solclient = SolClient::new(SolClientLogLevel::Notice);
    dotenv().ok();
    let host = std::env::var("SOLACE_HOST").unwrap();
    let username = std::env::var("SOLACE_USERNAME").unwrap();
    let password = std::env::var("SOLACE_PASSWORD").unwrap();
    let vpn = std::env::var("SOLACE_VPN").unwrap();
    let cache_name = std::env::var("SOLACE_CACHE_NAME").unwrap();
    match solclient {
        Ok(mut solclient) => {
            let event_recv = solclient.get_event_receiver();
            std::thread::spawn(move || loop {
                let event = event_recv.recv().unwrap();
                println!("event: {:?}", event);
            });
            let msg_recv = solclient.get_msg_receiver();
            std::thread::spawn(move || loop {
                match msg_recv.recv() {
                    Ok(msg) => {
                        tracing::info!(
                            "msg: {} {}",
                            msg.get_topic().unwrap(),
                            msg.get_sender_dt()
                                .unwrap_or(chrono::prelude::Utc::now())
                                .to_rfc3339(),
                            // msg.get_binary_attachment().unwrap()
                        );
                    }
                    Err(e) => {
                        tracing::error!("recv msg error: {}", e);
                    }
                }
            });
            let session_props = SessionProps::default()
                .host(&host)
                .vpn(&vpn)
                .username(&username)
                .password(&password)
                .client_name("cachetest")
                .connect_retries(1)
                .connect_timeout_ms(3000)
                .reapply_subscriptions(true)
                .compression_level(5);
            let r = solclient.connect(session_props);
            tracing::info!("r: {:?}", r);
            let cache_session_props = CacheSessionProps::default()
                .cache_name(&cache_name)
                .max_msgs(1)
                .request_reply_timeout(3000);
            let topic = "TIC/v1/STK/bcdmzpcr01/TSE/2890";
            let _res = solclient.send_cache_request(
                topic,
                100,
                cache_session_props,
                rsolace::types::SolClientCacheRequestFlags::LiveDataFlowThru,
            );
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
        Err(e) => {
            tracing::error!("Error creating SolClient: {}", e);
        }
    }
}
