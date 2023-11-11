use std::net::TcpListener;

use web_rust_prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:0";
    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    println!("server running on: http://{}:{}", address, port);
    run(listener)?.await
}
