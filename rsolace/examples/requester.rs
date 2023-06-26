use rsolace::solclient::{SessionProps, SolClient};
use rsolace::solmsg::SolMsg;
use rsolace::types::SolClientLogLevel;
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
                solclient.set_rx_msg_callback(|_, msg| {
                    tracing::info!(
                        "{} {:?}",
                        msg.get_topic().unwrap(),
                        msg.get_binary_attachment().unwrap()
                    );
                });
            }
            #[cfg(feature = "channel")]
            {
                let event_recv = solclient.get_event_clone_receiver();
                std::thread::spawn(move || loop {
                    let event = event_recv.recv().unwrap();
                    tracing::info!("{:?}", event);
                });
                let msg_recv = solclient.get_msg_clone_receiver();
                std::thread::spawn(move || loop {
                    let msg = msg_recv.recv().unwrap();
                    tracing::info!(
                        "{} {:?}",
                        msg.get_topic().unwrap(),
                        msg.get_binary_attachment().unwrap()
                    );
                });
            }

            let props = SessionProps::default()
                .host("localhost:55555")
                .vpn("sinopac")
                .username("shioaji")
                .password("shioaji111")
                .client_name("requester")
                .reapply_subscriptions(true)
                .connect_retries(1)
                .connect_timeout_ms(3000)
                .compression_level(0);
            let r = solclient.connect(props);
            tracing::info!("connect: {}", r);

            let mut msg = SolMsg::new().unwrap();
            msg.set_topic("api/v1/test");
            msg.set_delivery_to_one(true);
            let res = solclient.send_request(&msg, 5000);
            tracing::info!("send request msg: {:?}", res);
            tracing::info!("done");
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
}
