use axum::response::IntoResponse;

pub async fn admin_dashboard() -> impl IntoResponse {
    axum::http::StatusCode::OK
}
