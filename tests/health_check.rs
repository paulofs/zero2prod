/// Lauch the application
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();

    // We need to bring in `request`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to execure request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
