use std::net::TcpListener;

use axum::{http::StatusCode, routing::{get, IntoMakeService}, Router};
use hyper::server::conn::AddrIncoming;

/// Return `200 OK` if the API is running and is accessible
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// WARN: That's ugly
pub fn run(listener: TcpListener) -> hyper::Result<hyper::Server<AddrIncoming, IntoMakeService<Router>>>{
    let app = Router::new().route("/health_check", get(health_check));

    let server = axum::Server::from_tcp(listener)?
        .serve(app.into_make_service());
    Ok(server)
}

#[cfg(test)]
mod tests {
    use super::health_check;

    #[tokio::test]
    async fn health_ckeck_succeds() {
        let response = health_check().await;
        assert!(response.is_success())
    }
}
