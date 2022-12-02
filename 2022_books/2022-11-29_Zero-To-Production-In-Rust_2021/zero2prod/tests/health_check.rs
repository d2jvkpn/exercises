mod helpers;

use helpers::spawn_app_without_create;

#[tokio::test]
async fn health_check() {
    let app = spawn_app_without_create().await;

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/healthz", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
