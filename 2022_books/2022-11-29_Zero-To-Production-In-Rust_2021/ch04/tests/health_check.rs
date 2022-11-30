use ch04::{configuration, run};
use std::net::TcpListener;

// use sqlx::{Connection, PgConnection};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = configuration::open("configs/local.yaml").expect("Failed to read configuration");

    let mut conn =
        PgConnection::connect(&config.database).await.expect("Failed to connect to Postgres");

    let dbname = "newsletter_test_".to_owned() + &Uuid::new_v4().to_string();
    let dsn = config.database.clone().replace("newsletter", &dbname);

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, &dbname).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect(&dsn).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    let server = run(listener, pool.clone(), 0).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);

    TestApp { address, pool }
}

#[actix_rt::test]
async fn health_check() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    // Act
    let response = client
        .post(&format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
