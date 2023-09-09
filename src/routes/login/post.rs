use axum::{
    debug_handler,
    response::{IntoResponse, Response},
    Extension, Form,
};

use axum_sessions::extractors::WritableSession;
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    routes::error_chain_fmt,
};

#[tracing::instrument(
    skip(form, db_pool),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
    )]
#[debug_handler]
pub async fn login(
    Extension(db_pool): Extension<PgPool>,
    mut session: WritableSession,
    Form(form): Form<FormData>,
) -> Result<Response, Response> {
    let credentials = Credentials {
        username: form.username,
        password: form.password,
    };

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            session
                .insert("user_id", user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))
                .into_response();
            Ok((
                axum::http::StatusCode::SEE_OTHER,
                [(axum::http::header::LOCATION, "/admin/dashboard")],
            )
                .into_response())
        }
        // IDF how to deal with that e yet
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            // --- Response
            Err(login_redirect(e).into_response()) // Idk how to propagate that e yet
                                                   // ---
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn login_redirect(e: LoginError) -> Response {
    (
        axum::http::StatusCode::SEE_OTHER,
        [
            (axum::http::header::LOCATION, "/login"),
            (axum::http::header::SET_COOKIE, &format!("_flash={}", e)),
        ],
    )
        .into_response()
}
