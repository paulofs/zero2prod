/// Lauch the application
async fn spawn_app() -> std::io::Result<()> {
    zero2prod::run().await;
    Ok(())
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app().await.expect("Failed to spawn the app.");

    // We need to bring in `request`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execure request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
