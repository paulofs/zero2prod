use axum::{extract::Query, response::IntoResponse};

pub async fn login_form(query: Query<QueryParams>) -> impl IntoResponse {
    let error_html = match query.0.error {
        None => "".into(),
        Some(error_message) => format!("<p><i>{error_message}</i></p>"),
    };
    (
        axum::http::StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            ("text/html; charset=UTF-8"),
        )],
        axum::response::Html(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
> </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
> </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
        )),
    )
}

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: Option<String>,
}
