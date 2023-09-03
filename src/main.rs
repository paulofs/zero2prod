//! src/main.rs
use std::{net::TcpListener, time::Duration};
use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&configuration.database.connection_string())
        .await.expect("Failed to connect");

    let address = format!("0.0.0.0:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection)?.await
}
