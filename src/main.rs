use std::net::TcpListener;

use server::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed To bind random port.");

    run(listener)?.await
}
