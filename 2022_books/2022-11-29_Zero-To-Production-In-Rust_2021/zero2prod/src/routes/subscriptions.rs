use actix_web::{
    http::header::ContentType,
    web::{Data, Form, ReqData},
    HttpResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{self, PgPool};
use tracing::{self, info_span, Instrument};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeData {
    email: String,
    name: String,
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

    let mut resp = json!({"code": 0,"msg": "ok", "data": {}, "requestId": request_id});

    if form.email.is_empty() || form.name.is_empty() {
        resp["code"] = (-1).into();
        resp["msg"] = "email or name is unset".into();

        return HttpResponse::BadRequest().content_type(ContentType::json()).body(resp.to_string());
    }
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
