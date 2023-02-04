#![allow(dead_code)]
use super::{models::Course, response::Error};
use sqlx::{self, postgres::PgPool, query};

pub async fn get_courses_for_tutor(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, Error> {
    let result = query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5
        WHERE tutor_id = $1",
        tutor_id,
    )
    .fetch_all(pool)
    .await;

    let rows = match result {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let courses: Vec<Course> = rows
        .iter()
        .map(|v| Course {
            course_id: v.course_id,
            tutor_id: v.tutor_id,
            course_name: v.course_name.clone(),
            posted_time: v.posted_time,
        })
        .collect();

    match courses.len() {
        0 => Err(Error::NotFound("courses not found for tutor".into())),
        _ => Ok(courses),
    }
}

// TODO: wrap message and cause(anyhow::Error)
// go doc google.golang.org/grpc/codes.Internal
#[allow(dead_code)]
pub enum DBErr {
    InvalidArgument,
    NotFound,
    Conflict,
    PermissionDenied,
    Unauthenticated,
    Internal,
    Unkonwn,
}

pub async fn get_course_details(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, Error> {
    let row = query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5
        WHERE tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        course_id: row.course_id,
        tutor_id: row.tutor_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    })
}

pub async fn post_new_course(pool: &PgPool, course: Course) -> Result<Course, Error> {
    let row = query!(
        "INSERT INTO ezy_course_c5 (tutor_id, course_name) VALUES ($1, $2)
        RETURNING course_id, tutor_id, course_name, posted_time",
        course.tutor_id,
        course.course_name,
    )
    .fetch_one(pool)
    .await?;

    Ok(Course {
        course_id: row.course_id,
        tutor_id: row.tutor_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    })
}
