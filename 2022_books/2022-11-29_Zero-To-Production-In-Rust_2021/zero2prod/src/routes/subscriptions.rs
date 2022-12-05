use actix_web::{
    http::header::ContentType,
    web::{Data, Form, ReqData},
    HttpResponse,
};
use chrono::Utc;
use email_address::*;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx::{self, PgPool};
use tracing::{self, info_span, Instrument};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeData {
    email: String,
    name: String,
}

impl SubscribeData {
    fn valid(&self) -> Result<(), String> {
        let (name, email) = (&self.name, &self.email);

        if name.trim().is_empty() || email.trim().is_empty() {
            return Err("name or empay is empty".into());
        }

        if name.graphemes(true).count() > 256 || email.graphemes(true).count() > 256 {
            return Err("name or empay is too long".into());
        }

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        if name.chars().any(|g| forbidden_characters.contains(&g)) {
            return Err("name contains forbidden characters".into());
        }

        if !EmailAddress::is_valid(email) {
            return Err("email contains forbidden characters".into());
        }

        Ok(())
    }
}

pub async fn subscribe(
    pool: Data<PgPool>,
    form: Form<SubscribeData>,
    request_id: ReqData<Uuid>,
) -> HttpResponse {
    let request_id = request_id.into_inner();
    let subscriber_id = Uuid::new_v4();

    let req_span = info_span!(
        "Adding a new subscriber.",
        %request_id,
        email = %form.email,
        name= %form.name,
    );
    let _req_span_guard = req_span.enter();

    let mut resp = serde_json::json!({"code": 0,"msg": "ok", "data": {}, "requestId": request_id});

    if let Err(e) = form.valid() {
        resp["code"] = (-1).into();
        resp["msg"] = serde_json::Value::String(e);

        return HttpResponse::BadRequest().content_type(ContentType::json()).body(resp.to_string());
    };
    // TODO email length and name length check

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
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
    .instrument(query_span)
    .await;

    let err = match result {
        // return HttpResponse::Ok().finish()
        Ok(_) => {
            return HttpResponse::Ok().content_type(ContentType::json()).body(resp.to_string())
        }
        Err(e) => e,
    };

    if let sqlx::Error::Database(e) = &err {
        if e.code().unwrap() == "23505" {
            // return HttpResponse::Ok().finish();
            resp["msg"] = "you have already subscribed".into();
            tracing::warn!("The email has already been subscribed.");

            return HttpResponse::Conflict()
                .content_type(ContentType::json())
                .body(resp.to_string());
        }
    }

    // println!("Failed to execute query: {:?}", err);
    tracing::error!("Failed to execute query: {:?}", err);

    // HttpResponse::InternalServerError().finish()
    resp["code"] = 1.into();
    resp["msg"] = "internal server error".into();

    HttpResponse::InternalServerError().content_type(ContentType::json()).body(resp.to_string())
}
