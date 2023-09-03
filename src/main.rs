use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0")
        .expect("Failed to bind random port");
    run(listener)?.await
}
