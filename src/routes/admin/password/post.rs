use axum::{
    response::{IntoResponse, Response},
    Form, debug_handler,
};
use axum_sessions::extractors::ReadableSession;
use secrecy::Secret;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}
#[debug_handler]
pub async fn change_password(session: ReadableSession, Form(form): Form<FormData>) -> Response {
    if session.get::<uuid::Uuid>("user_id").is_none() {
        return (
            axum::http::StatusCode::SEE_OTHER,
            [(axum::http::header::LOCATION, "/login")],
        )
            .into_response();
    }
    todo!()
}
