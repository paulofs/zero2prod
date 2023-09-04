//! src/routes/subscriptions.rs
// See: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    Form,
};
use sqlx::{
    pool::PoolConnection,
    types::{chrono::Utc, Uuid},
    Acquire, PgPool, Postgres,
};
use unicode_segmentation::UnicodeSegmentation;

/// Returns `true` if the input satisfies all our validation constraints
/// on subscriber names, `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();

    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two cha-
    // racters // (`a` and `̊`).
    //
    // `graphemes` returns a interator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme defini-
    // tion set, the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;

    // Iterate over all characters in the input `s` to check if any of them
    // matches one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    // Return `false` if any or our conditions have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

/// Creates a span at the beginning of the function invocation and automatically ataches all
/// arguments passed to the function to the context of the span
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection_pool),
    fields(
        //request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    DatabaseConnection(mut connection_pool): DatabaseConnection,
    Form(form): Form<FormData>,
) -> StatusCode {
    match insert_subscriber(&mut connection_pool, &form).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tracing::instrument(
    name = "Saving a new subscriber details in the database",
    skip(form, connection_pool)
)]
pub async fn insert_subscriber(
    connection_pool: &mut PoolConnection<Postgres>,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    let connection = connection_pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

// we can extract the connection pool with `State`
pub async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}
/*
async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)
}
*/
/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
