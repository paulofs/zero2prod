use crate::helpers::{assert_is_redirect_to, spawn_app};

#[tokio::test]
async fn you_must_be_logged_in_to_access_the_admin_dashboard() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_admin_dashboard().await;

    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn logout_clears_session_state() {
    // Arrange
    let app = spawn_app().await;

    // Act - Part 1 - Login
    let login_body = serde_json::json!({ "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - Part 2 - Follow the redirect
    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Wellcome {}", app.test_user.username)));

    // Act - Part 3 - Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    // Act - Part 4 - Follow the redirect TODO: Fix that cookie handling
    // let html_page = app.get_login_html().await;
    //dbg!(&html_page);
    //assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));

    // Act - Part 5 - Attempt to load admin panel
    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;
    let new_password = uuid::Uuid::new_v4().to_string();
    // Act - Part 1 - Login
    let login_body = serde_json::json!({ "username": &app.test_user.username,
        "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
    // Act - Part 2 - Change password
    let response = app
        .post_change_password(&serde_json::json!({
                    "current_password": &app.test_user.password,
                    "new_password": &new_password,
                    "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");
    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));
    // Act - Part 4 - Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");
    // Act - Part 5 - Follow the redirect
    // let html_page = app.get_login_html().await;
    // assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));
    // Act - Part 6 - Login using the new password
    let login_body = serde_json::json!({ "username": &app.test_user.username,
        "password": &new_password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}
