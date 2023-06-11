use rsolace::solclient::SolClient;
use rsolace::types::{SolClientLogLevel, SolClientSubscribeFlags};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    let solclient = SolClient::new(SolClientLogLevel::Notice);
    match solclient {
        Ok(mut solclient) => {
            solclient.set_rx_event_callback(|event| {
                tracing::info!("{:?}", event);
            });
            let r = solclient.connect(
                "218.32.76.102:80",
                "sinopac",
                "shioaji",
                "shioaji111",
                Some("c1"),
                None,
                None,
            );
            // solclient.set_rx_msg_callback(func)
            solclient.subscribe_ext("TIC/v1/test1", SolClientSubscribeFlags::RequestConfirm);
            solclient.subscribe_ext("TIC/v1/test2", SolClientSubscribeFlags::RequestConfirm);

            println!("connect: {}", r);
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
    let solclient2 = SolClient::new(SolClientLogLevel::Notice);
    match solclient2 {
        Ok(mut solclient) => {
            let r = solclient.connect(
                "218.32.76.102:80",
                "sinopac",
                "shioaji",
                "shioaji111",
                Some("c2"),
                None,
                None,
            );
            println!("connect: {}", r);
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
}
