use axum::{response::Response, Form};
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}

pub async fn change_password(Form(form): Form<FormData>) -> Response {
    todo!()
}
