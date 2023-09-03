//! src/routes/subscriptions.rs
// See: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    Form,
};
use sqlx::{
    types::{chrono::Utc, Uuid},
    Acquire, PgPool,
};

// Start simple: WARN: we always return a 200 OK
pub async fn subscribe(
    DatabaseConnection(mut connection_pool): DatabaseConnection,
    Form(form_data): Form<FormData>,
) -> StatusCode {
    let connection = connection_pool.acquire().await.unwrap();
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        form_data.email,
        form_data.name,
        Utc::now()
    )
    .execute(connection)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR},
    }
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
