//! src/routes/subscriptions.rs
use axum::{http::StatusCode, Form};

// Start simple: WARN: we always return a 200 OK
pub async fn subscribe(Form(form_data): Form<FormData>) -> StatusCode {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
