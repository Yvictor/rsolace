use rsolace::solclient::{SessionProps, SolClient};
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
                solclient.set_rx_msg_callback(|_, msg| {
                    tracing::info!(
                        "{} {} {:?}",
                        msg.get_topic().unwrap(),
                        msg.get_sender_time()
                            .unwrap_or(chrono::prelude::Utc::now())
                            //.format("%Y-%m-%d %H:%M:%S%.3f")
                            // .to_string(),
                            .to_rfc3339(),
                        msg.get_binary_attachment().unwrap()
                    );
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
                        "{} {} {:?}",
                        msg.get_topic().unwrap(),
                        msg.get_sender_time()
                            .unwrap_or(chrono::prelude::Utc::now())
                            .to_rfc3339(),
                        msg.get_binary_attachment().unwrap()
                    );
                });
            }

            let props = SessionProps::default()
                .host("218.32.76.102:80")
                .vpn("sinopac")
                .username("shioaji")
                .password("shioaji111")
                .reapply_subscriptions(true)
                .connect_retries(1)
                .connect_timeout_ms(3000)
                .compression_level(5);

            let r = solclient.connect(props);
            tracing::info!("connect: {}", r);

            // solclient.set_rx_msg_callback(func)
            solclient.subscribe_ext(
                "TIC/v1/STK/*/TSE/2230",
                SolClientSubscribeFlags::RequestConfirm,
            );
            solclient.subscribe_ext(
                "QUO/v1/STK/*/TSE/2330",
                SolClientSubscribeFlags::RequestConfirm,
            );
            std::thread::sleep(std::time::Duration::from_secs(5));
            let mut msg = SolMsg::new().unwrap();
            msg.set_topic("api/v1/test");
            let rt = solclient.send_msg(&msg);
            tracing::info!("send msg: {:?}", rt);
            let mut msgs = vec![SolMsg::new().unwrap(), SolMsg::new().unwrap()];
            for (i, msg) in msgs.iter_mut().enumerate() {
                msg.set_topic(format!("api/v1/test/{}", i).as_str());
            }
            let rt = solclient.send_multiple_msg(&msgs);
            tracing::info!("send multiple msg: {:?}", rt);
            let mut msg = SolMsg::new().unwrap();
            msg.set_topic("api/v1/test");
            let res = solclient.send_request(&msg, 0);
            tracing::info!("send request msg: {:?}", res);
            tracing::info!("done");
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
    // let solclient2 = SolClient::new(SolClientLogLevel::Notice);
    // match solclient2 {
    //     Ok(mut solclient) => {
    //         let r = solclient.connect(
    //             "218.32.76.102:80",
    //             "sinopac",
    //             "shioaji",
    //             "shioaji111",
    //             Some("c2"),
    //             None,
    //             None,
    //         );
    //         println!("connect: {}", r);
    //     }
    //     Err(e) => {
    //         println!("error: {}", e)
    //     }
    // }
    // std::thread::sleep(std::time::Duration::from_secs(5));
}
