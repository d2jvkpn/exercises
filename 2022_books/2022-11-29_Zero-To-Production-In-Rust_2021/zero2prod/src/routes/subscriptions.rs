use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{self, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeData {
    email: String,
    name: String,
}

pub async fn subscribe(pool: web::Data<PgPool>, form: web::Form<SubscribeData>) -> HttpResponse {
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

    let mut resp = json!({"code": 0,"msg": "ok"});

    let err = match result {
        Ok(_) => {
            // return HttpResponse::Ok().finish()
            return HttpResponse::Ok().content_type(ContentType::json()).body(resp.to_string());
        }
        Err(e) => e,
    };

    // println!("Failed to execute query: {:?}", err);

    if let sqlx::Error::Database(e) = err {
        if e.code().unwrap() == "23505" {
            // return HttpResponse::Ok().finish();
            resp["msg"] = "you have already subscribed".into();

            return HttpResponse::Conflict()
                .content_type(ContentType::json())
                .body(resp.to_string());
        }
    }

    // HttpResponse::InternalServerError().finish()
    resp["code"] = 1.into();
    resp["msg"] = "internal server error".into();

    HttpResponse::InternalServerError().content_type(ContentType::json()).body(resp.to_string())
}
