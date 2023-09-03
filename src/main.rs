//! src/main.rs
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("0.0.0.0:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener)?.await
}
