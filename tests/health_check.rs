#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We need to bring in reqwest to perform
    // our HTTP requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> () {
    let server = rusty_project::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
