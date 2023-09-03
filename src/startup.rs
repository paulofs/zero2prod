//! src/startup.rs
// REMOVE EVERYTHING BELLOW
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post, IntoMakeService},
    Router,
};
use hyper::server::conn::AddrIncoming;
// WARN: That's ugly
pub fn run(
    listener: TcpListener,
) -> hyper::Result<hyper::Server<AddrIncoming, IntoMakeService<Router>>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
