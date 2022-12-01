use zero2prod::{configuration, run};
use std::net::TcpListener;

// use sqlx::{Connection, PgConnection};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
    pub dbname: String,
}

// connect to database in config
async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = configuration::open("configs/local.yaml").expect("Failed to read configuration");

    let pool = PgPool::connect(&config.database).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    let server = run(listener, pool.clone(), 0).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);

    TestApp { address, pool, dbname: config.database.clone() }
}

// create a temporary database
async fn spawn_app_tmpdb() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = configuration::open("configs/local.yaml").expect("Failed to read configuration");

    let mut conn =
        PgConnection::connect(&config.database).await.expect("Failed to connect to Postgres");

    let dbname = "newsletter_test_".to_owned() + &Uuid::new_v4().to_string();
    let dsn = config.database.clone().replace("newsletter", &dbname);

    let db_list = sqlx::query!(
        r#"SELECT datname FROM pg_database
	WHERE datistemplate = false AND datname LIKE 'newsletter_test_%';"#
    )
    .fetch_all(&mut conn)
    .await
    .expect("Failed to fetch test databases.");

    for db in db_list {
        _ = conn.execute(format!(r#"DROP DATABASE "{}";"#, &db.datname).as_str()).await;
    }

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, &dbname).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect(&dsn).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    let server = run(listener, pool.clone(), 0).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);

    TestApp { address, pool, dbname }
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
    let app = spawn_app_tmpdb().await;
    let client = reqwest::Client::new();
    let path = format!("{}/open/subscribe", &app.address);
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    // Act
    let response = client
        .post(&path)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let response = client
        .post(&path)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(409, response.status().as_u16());

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&path)
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
