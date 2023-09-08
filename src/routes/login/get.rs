use axum::{debug_handler, response::IntoResponse};

#[debug_handler]
pub async fn login_form() -> impl IntoResponse {
    let error_html: String = todo!();

    (
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=UTF-8")],
        ("/* HTML */"),
    )
}
