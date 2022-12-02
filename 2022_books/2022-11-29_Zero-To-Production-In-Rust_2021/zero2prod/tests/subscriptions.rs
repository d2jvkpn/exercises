mod helpers;

use crate::helpers::spawn_app_create_db;

#[tokio::test]
async fn subscribe() {
    // Arrange
    let app = spawn_app_create_db().await;
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
