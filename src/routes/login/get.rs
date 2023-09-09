use axum::{
    debug_handler,
    response::{Html, IntoResponse},
};
use axum_extra::extract::CookieJar;

#[debug_handler]
pub async fn login_form(cookiejar: CookieJar) -> impl IntoResponse {
    let error_html = match cookiejar.get("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };

    (
        [
            (axum::http::header::CONTENT_TYPE, "text/html; charset=UTF-8"),
            (axum::http::header::SET_COOKIE, "_flash=;Max-Age=0"),
        ],
        Html(format!(
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
            > 
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            > 
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
        )),
    )
}
