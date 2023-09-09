use axum::response::{Html, IntoResponse, Response};
use axum_sessions::extractors::ReadableSession;

pub async fn change_password_form(session: ReadableSession) -> Response {
    if session.get::<uuid::Uuid>("user_id").is_none() {
        return (
                    axum::http::StatusCode::SEE_OTHER,
                    [(axum::http::header::LOCATION, "/login")]
               ).into_response();
    }
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/html; charset=UTF-8")],
        Html(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Change Password</title>
</head>
<body>
    <form action="/admin/password" method="post">
        <label>Current password
            <input
                type="password"
                placeholder="Enter current password"
                name="current_password"
            >
        </label>
        <br>
        <label>New password
            <input
                type="password"
                placeholder="Enter new password"
                name="new_password"
            >
        </label>
        <br>
        <label>Confirm new password
            <input
                type="password"
                placeholder="Type the new password again"
                name="new_password_check"
            >
        </label>
        <br>
        <button type="submit">Change password</button>
</form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        ),
    )
        .into_response()
}
