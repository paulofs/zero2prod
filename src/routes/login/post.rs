use axum::{response::IntoResponse, Form};
use secrecy::Secret;


pub async fn login(form: Form<FormData>) -> impl IntoResponse {
    (
        axum::http::StatusCode::SEE_OTHER,
        [
            (axum::http::header::LOCATION, "/"),
        ]
    )
}

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}
