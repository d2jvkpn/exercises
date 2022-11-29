use a01::run;

#[actix_rt::test]
async fn health_check_works() {
    spawn_app("0.0.0.0:8000");

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app(addr: &str) {
    let server = run(addr).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);
}
