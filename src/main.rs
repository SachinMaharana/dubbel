use dubbel::run;
use std::net::TcpListener;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind to random port");
    // let port = listener.local_addr().unwrap().port();
    run(listener)?.await
}