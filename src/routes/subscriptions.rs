//! src/routes/subscriptions.rs
// See: https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs
use axum::{http::StatusCode, Extension, Form};
use sqlx::{
    types::{chrono::Utc, Uuid},
    Pool, Postgres,
};

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

/// Creates a span at the beginning of the function invocation and automatically ataches all
/// arguments passed to the function to the context of the span
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool),
    fields(
        //request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Form(form): Form<FormData>,
) -> StatusCode {
    let new_subscriber = match form.try_into() {
        Ok(form) => form,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match insert_subscriber(&db_pool, &new_subscriber).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

#[tracing::instrument(
    name = "Saving a new subscriber details in the database",
    skip(new_subscriber, db_pool)
)]
pub async fn insert_subscriber(
    db_pool: &Pool<Postgres>,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    // let mut connection = connection_pool.acquire().await.unwrap();
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at, status)
    VALUES ($1, $2, $3, $4, 'confirmed')
            "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(db_pool)
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
