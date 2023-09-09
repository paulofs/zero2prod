use anyhow::Context;
use axum::{
    debug_handler,
    response::{Html, IntoResponse, Response},
    Extension,
};
use axum_sessions::extractors::ReadableSession;
use sqlx::PgPool;
use uuid::Uuid;
#[debug_handler]
pub async fn admin_dashboard(
    Extension(db_pool): Extension<PgPool>,
    session: ReadableSession,
) -> Response {
    let username = if let Some(user_id) = session.get::<Uuid>("user_id") {
        get_username(user_id, &db_pool)
            .await
            .expect("Failed to get the username")
    } else {
        return (
            axum::http::StatusCode::SEE_OTHER,
            [(axum::http::header::LOCATION, "/login")],
        )
            .into_response();
    };
    (
        axum::http::StatusCode::OK,
        Html(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Admin dashboard</title>
</head>
<body>
    <p>Wellcome {username}!</p>
    <p>Available actions:</p>
    <ol>
        <li><a href="/admin/password">Change password</a></li>
    </ol>
</body>
</html>"#
        )),
    )
        .into_response()
}

#[tracing::instrument(name = "Get username", skip(db_pool))]
async fn get_username(user_id: Uuid, db_pool: &PgPool) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(db_pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}
