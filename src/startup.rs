//! src/startup.rs
// see: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs 
use std::net::TcpListener;

use crate::routes::{health_check, subscribe, using_connection_pool_extractor};
use axum::{
    routing::{get, IntoMakeService},
    Router,
};
use hyper::server::conn::AddrIncoming;
use sqlx::{Pool, Postgres};
// WARN: That's ugly
pub fn run(
    listener: TcpListener,
    connection: Pool<Postgres>
) -> hyper::Result<hyper::Server<AddrIncoming, IntoMakeService<Router>>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", get(using_connection_pool_extractor).post(subscribe))
        .with_state(connection);

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}


