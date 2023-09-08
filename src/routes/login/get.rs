use axum::response::IntoResponse;

pub async fn login_form() -> impl IntoResponse {
    (
        axum::http::StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            ("text/html; charset=UTF-8"),
        )],
        include_str!("login.html"),
    )
}
