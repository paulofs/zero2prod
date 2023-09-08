
use axum::{debug_handler, response::{IntoResponse, Response}, Extension, Form};
use hmac::Mac;

use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    routes::error_chain_fmt,
};

#[tracing::instrument(
    skip(form, db_pool, secret),
    fields(username=tracing::field::Empty, user_id=tracing::field::Empty)
    )]
#[debug_handler]
pub async fn login(
    Extension(db_pool): Extension<PgPool>,
    Extension(secret): Extension<Secret<String>>,
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
            Ok((
                axum::http::StatusCode::SEE_OTHER,
                [(axum::http::header::LOCATION, "/")],
            )
                .into_response())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            let query_string = format!("error={}", urlencoding::Encoded::new(e.to_string()));
            let hmac_tag = {
                let mut mac =
                    hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.expose_secret().as_bytes())
                        .unwrap();
                mac.update(query_string.as_bytes());
                mac.finalize().into_bytes()
            };
            // --- Response
            let response = (
                axum::http::StatusCode::SEE_OTHER,
                [(
                    axum::http::header::LOCATION,
                    format!("/login?{}&tag={:x}", query_string, hmac_tag),
                )],
            );
            Err(response.into_response())
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
