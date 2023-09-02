use axum::{http::StatusCode, routing::get, Router};

/// Return `200 OK` if the API is running and is accessible
async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run() {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
