#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    // Launch our application in the background ~somehow~
}

async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async {
        zero2prod::run(listener).await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    format!("http://127.0.0.1:{}", port)
}
