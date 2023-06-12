use rsolace::solclient::{SessionProps, SolClient};
use rsolace::types::{SolClientLogLevel, SolClientSubscribeFlags};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let solclient = SolClient::new(SolClientLogLevel::Notice);
    match solclient {
        Ok(mut solclient) => {
            solclient.set_rx_event_callback(|event| {
                tracing::info!("{:?}", event);
            });
            let props = SessionProps::default()
                .host("218.32.76.102:80")
                .vpn("sinopac")
                .username("shioaji")
                .password("shioaji111")
                .reapply_subscriptions(true)
                .connect_retries(1)
                .connect_timeout(3000)
                .compression_level(5);

            let r = solclient.connect(props);
            tracing::info!("connect: {}", r);

            // solclient.set_rx_msg_callback(func)
            solclient.subscribe_ext("TIC/v1/test1", SolClientSubscribeFlags::RequestConfirm);
            solclient.subscribe_ext("TIC/v1/test2", SolClientSubscribeFlags::RequestConfirm);
            std::thread::sleep(std::time::Duration::from_secs(5));
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
