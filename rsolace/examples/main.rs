use rsolace::solclient::SolClient;
use rsolace::types::SolClientLogLevel;

fn main() {
    let solclient = SolClient::new(SolClientLogLevel::Notice);
    match solclient {
        Ok(mut solclient) => {
            let r = solclient.connect(
                "218.32.76.102:80",
                "sinopac",
                "shioaji",
                "shioaji111",
                Some("c1"),
                None,
                None,
            );
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
}
