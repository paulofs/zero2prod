use axum::{
    debug_handler,
    response::{IntoResponse, Response},
    Form,
};
use axum_sessions::extractors::ReadableSession;
use secrecy::{ExposeSecret, Secret};

use crate::utils::see_other;

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}
#[debug_handler]
pub async fn change_password(session: ReadableSession, Form(form): Form<FormData>) -> Response {
    if session.get::<uuid::Uuid>("user_id").is_none() {
        return see_other("/login");
    }
    if form.new_password.expose_secret() != form.new_password_check.expose_secret() {
        return (
            axum::http::StatusCode::SEE_OTHER,
            [
                (axum::http::header::LOCATION, "/admin/password"),
                (
                    axum::http::header::SET_COOKIE,
                    &format!(
                        "_flash={}",
                        "You entered two different new passwords - the field values must match."
                    ),
                ),
            ],
        )
            .into_response();
    }
    todo!()
}
