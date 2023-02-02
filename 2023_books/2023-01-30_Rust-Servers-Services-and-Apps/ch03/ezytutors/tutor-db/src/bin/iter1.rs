// use sqlx::types::chrono::{DateTime, Utc};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::{env, io};

#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<DateTime<Utc>>,
}
// TODO: chrono, DateTime with different timezones

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is unset");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    let course_rows =
        sqlx::query!(r#"select course_id, tutor_id, course_name, posted_time from ezy_course_c4"#,)
            .fetch_all(&db_pool)
            .await
            .unwrap();

    let mut courses_list = vec![];

    for row in course_rows {
        courses_list.push(Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name,
            posted_time: row.posted_time,
        });
    }

    println!("Courses: {:?}", courses_list);

    Ok(())
}
