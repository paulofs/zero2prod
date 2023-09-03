//! src/routes/health_check.rs
use axum::http::StatusCode;

/// Return `200 OK` if the API is running and is accessible
pub async fn health_check() -> StatusCode {
    StatusCode::OK
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
