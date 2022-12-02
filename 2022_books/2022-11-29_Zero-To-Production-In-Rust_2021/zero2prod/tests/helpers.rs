#![allow(dead_code)]

use std::net::TcpListener;
use zero2prod::{configuration::open_yaml, run};

// use sqlx::{Connection, PgConnection};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

// connect to database in config
pub async fn spawn_app_without_create() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = open_yaml("configs/local.yaml").expect("Failed to read configuration");

    let pool = PgPool::connect(&config.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    let server = run(listener, pool.clone(), Default::default()).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);

    TestApp { address, pool }
}

// create a temporary database
pub async fn spawn_app_create_db() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = open_yaml("configs/local.yaml").expect("Failed to read configuration");

    // remove temporary databases newsletter_test_*
    let mut conn = PgConnection::connect(&config.database.to_string())
        .await
        .expect("Failed to connect to Postgres");

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

    // create a temporary db
    let dbname = "newsletter_test_".to_owned() + &Uuid::new_v4().to_string();
    let dsn = config.database.conn.clone() + "/" + &dbname;

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, &dbname).as_str())
        .await
        .expect("Failed to create database.");

    let pool = PgPool::connect(&dsn).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    let server = run(listener, pool.clone(), config).expect("Failed to bind addrress");
    let _ = tokio::spawn(server);

    TestApp { address, pool }
}
