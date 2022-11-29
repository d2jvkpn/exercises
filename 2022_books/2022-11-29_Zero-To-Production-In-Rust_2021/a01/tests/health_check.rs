use a01::{run, run2};
use std::net::TcpListener;

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

#[actix_rt::test]
async fn health_check_works_v2() {
    let address = spawn_app2();

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app2() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = run2(listener).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
