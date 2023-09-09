use axum::response::{IntoResponse, Response};

// Return an opaque 500 while preserving the error root's cause for logging.
pub fn e500<T>(e: T) -> (axum::http::StatusCode, String)
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", e),
    )
}

pub fn see_other(location: &str) -> Response {
    (
        axum::http::StatusCode::SEE_OTHER,
        [(axum::http::header::LOCATION, location)],
    )
        .into_response()
}
