use rsolace::solclient::SolClient;
use rsolace::SessionProps;
use rsolace::solmsg::SolMsg;
use rsolace::types::{SolClientLogLevel, SolClientSubscribeFlags};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let solclient = SolClient::new(SolClientLogLevel::Notice);
    match solclient {
        Ok(mut solclient) => {
            #[cfg(feature = "raw")]
            {
                solclient.set_rx_event_callback(|_, event| {
                    tracing::info!("{:?}", event);
                });
                solclient.set_rx_msg_callback(|solclient, msg| {
                    tracing::info!(
                        "{}",
                        msg.get_topic().unwrap(),
                        // msg.get_binary_attachment().unwrap()
                    );
                    let mut reply_msg = SolMsg::new().unwrap();
                    reply_msg.set_reply_topic("api/v1/test");
                    // reply_msg.set_topic("api/v1/test");
                    let rt = solclient.send_reply(&msg, &reply_msg);
                    tracing::info!("reply: {:?}", rt);
                });
            }
            #[cfg(feature = "channel")]
            {
                let event_recv = solclient.get_event_receiver();
                std::thread::spawn(move || loop {
                    let event = event_recv.recv().unwrap();
                    tracing::info!("{:?}", event);
                });
                let msg_recv = solclient.get_msg_receiver();
                std::thread::spawn(move || loop {
                    let msg = msg_recv.recv().unwrap();
                    tracing::info!(
                        "{}",
                        msg.get_topic().unwrap(),
                        // msg.get_binary_attachment().unwrap()
                    );
                    let mut reply_msg = SolMsg::new().unwrap();
                    reply_msg.set_reply_topic("api/v1/test");
                    // reply_msg.set_topic("api/v1/test");
                    // let rt = solclient.send_reply(&msg, &reply_msg);
                    // tracing::info!("reply: {:?}", rt);
                });
            }

            let props = SessionProps::default()
                .host("localhost:55555")
                .vpn("sp-intra")
                .username("swrelay")
                .password("srwespwlsawy111")
                .client_name("replier")
                .reapply_subscriptions(true)
                .connect_retries(1)
                .connect_timeout_ms(3000)
                .compression_level(0);

            let r = solclient.connect(props);
            tracing::info!("connect: {}", r);

            solclient.subscribe_ext("api/v1/test", SolClientSubscribeFlags::RequestConfirm);
            std::thread::sleep(std::time::Duration::from_secs(30));
            tracing::info!("done");
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
}
