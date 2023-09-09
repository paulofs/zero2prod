// TODO: Refactor a LOT of these things
use axum::{
    debug_handler,
    response::{IntoResponse, Response},
    Extension, Form,
};
use axum_sessions::extractors::ReadableSession;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    routes::admin::dashboard::get_username,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    current_password: Secret<String>,
    new_password: Secret<String>,
    new_password_check: Secret<String>,
}
#[debug_handler]
pub async fn change_password(
    Extension(db_pool): Extension<PgPool>,
    session: ReadableSession,
    Form(form): Form<FormData>,
) -> Response {
    let user_id = session.get::<uuid::Uuid>("user_id");
    if user_id.is_none() {
        return see_other("/login");
    };
    let user_id = user_id.unwrap();
    let username = get_username(user_id, &db_pool)
        .await
        .expect("Failed to get username");

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
    };
    let credentials = Credentials {
        username,
        password: form.current_password,
    };
    if let Err(e) = validate_credentials(credentials, &db_pool).await {
        return match e {
            AuthError::InvalidCredentials(_) => {
                return (
                    axum::http::StatusCode::SEE_OTHER,
                    [
                        (axum::http::header::LOCATION, "/admin/password"),
                        (
                            axum::http::header::SET_COOKIE,
                            &format!("_flash={}", "The current password is incorrect."),
                        ),
                    ],
                )
                    .into_response();
            }
            AuthError::UnexpectedError(_) => e500(e).into_response(),
        };
    }
    todo!()
}
