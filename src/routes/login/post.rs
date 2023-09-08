use axum::{response::IntoResponse, Extension, Form};
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, Credentials},
    routes::error_chain_fmt,
};

#[tracing::instrument(
    skip(form, db_pool),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
    )]
pub async fn login(
    Extension(db_pool): Extension<PgPool>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let credentials = Credentials {
        username: form.username,
        password: form.password,
    };

    tracing::Span::current().record("username", &tracing::field::display(&credentials.username));

    match validate_credentials(credentials, &db_pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
            (
                axum::http::StatusCode::SEE_OTHER,
                [(axum::http::header::LOCATION, "/")],
            )
        }
        Err(_) => todo!(),
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

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        match self {
            LoginError::UnexpectedError(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            LoginError::AuthError(_) => axum::http::StatusCode::UNAUTHORIZED.into_response(),
        }
    }
}
