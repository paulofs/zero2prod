//! src/startup.rs
// REMOVE EVERYTHING BELLOW
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post, IntoMakeService},
    Router, Extension,
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
        .route("/subscriptions", post(subscribe))
        .layer(Extension(connection));

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
