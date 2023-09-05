//! src/startup.rs
// see: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};
use axum::{
    routing::{get, post, IntoMakeService},
    Extension, Router,
};
use hyper::server::conn::AddrIncoming;
use sqlx::{Pool, Postgres};

use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
// WARN: That's ugly
pub fn run(
    listener: TcpListener,
    db_pool: Pool<Postgres>,
) -> hyper::Result<hyper::Server<AddrIncoming, IntoMakeService<Router>>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(db_pool))
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        );
    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());
    Ok(server)
}
