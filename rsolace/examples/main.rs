use rsolace::solclient::{SolClient, SolClientLogLevel};

fn main() {
    let solclient = SolClient::new(SolClientLogLevel::Notice);
    match solclient {
        Ok(mut solclient) => {
            let r = solclient.connect(
                "218.32.76.102:80",
                "sinopac",
                "shioaji",
                "shioaji111",
                None,
                None,
                None,
            );
            println!("connect: {}", r);
        }
        Err(e) => {
            println!("error: {}", e)
        }
    }
}
