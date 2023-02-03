#![allow(dead_code)]
use super::models::Course;
use sqlx::{self, postgres::PgPool, query};

pub async fn get_courses_for_tutor(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    let rows = query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4
        WHERE tutor_id = $1",
        tutor_id,
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()
        .map(|v| Course {
            course_id: v.course_id,
            tutor_id: v.tutor_id,
            course_name: v.course_name.clone(),
            posted_time: v.posted_time,
        })
        .collect()
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

pub async fn get_course_details(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    let row = query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4
        WHERE tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_id: row.course_id,
        tutor_id: row.tutor_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    }
}

pub async fn get_course_details_v2(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, DBErr> {
    let result = query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4
        WHERE tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
    .fetch_one(pool)
    .await;

    let err = match result {
        Ok(row) => {
            return Ok(Course {
                course_id: row.course_id,
                tutor_id: row.tutor_id,
                course_name: row.course_name,
                posted_time: row.posted_time,
            })
        }
        Err(e) => e,
    };

    match err {
        sqlx::Error::RowNotFound => Err(DBErr::NotFound),
        _ => Err(DBErr::Internal),
    }
}

pub async fn post_new_course(pool: &PgPool, course: Course) -> Course {
    let row = query!(
        "INSERT INTO ezy_course_c4 (tutor_id, course_name) VALUES ($1, $2)
        RETURNING course_id, tutor_id, course_name, posted_time",
        course.tutor_id,
        course.course_name,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_id: row.course_id,
        tutor_id: row.tutor_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    }
}
