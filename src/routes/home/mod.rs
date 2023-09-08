use axum::http;
use axum::response::IntoResponse;

pub async fn home() -> impl IntoResponse {
    (
        http::StatusCode::OK,
        [
            (http::header::SERVER, "axum"),
            (http::header::CONTENT_TYPE, "text/html; charset=UTF-8"),
        ],
        include_str!("home.html"),
    )
}
