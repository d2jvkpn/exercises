use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{Error::Database, PgPool};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(pool: web::Data<PgPool>, form: web::Form<FormData>) -> HttpResponse {
    let subscriber_id = Uuid::new_v4();

    let result = sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
  VALUES ($1, $2, $3, $4)
"#,
        subscriber_id,
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .await;

    let err = match result {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(e) => e,
    };

    println!("Failed to execute query: {:?}", err);

    if let Database(e) = err {
        if e.code().unwrap() == "23505" {
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::InternalServerError().finish()
}
