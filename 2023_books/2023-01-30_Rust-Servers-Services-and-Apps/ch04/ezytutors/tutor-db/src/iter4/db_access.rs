#![allow(dead_code)]
use super::{models::Course, response::Error};
use sqlx::{self, error::Error as SQLxError, postgres::PgPool, query};

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
        Err(e) => return Err(e.into()), // sqlx::error::Error to response::Error
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
    .await
    .map_err(|e| {
        // println!("!!! {:?}", e);
        match e {
            SQLxError::RowNotFound => Error::NotFound("course not exists".into()),
            _ => Error::DBError(e.to_string()),
        }
    })?;

    Ok(Course {
        course_id: row.course_id,
        tutor_id: row.tutor_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    })
}

pub async fn post_new_course(pool: &PgPool, course: Course) -> Result<Course, Error> {
    if course.course_id == 0 {
        let row = query!(
            "INSERT INTO ezy_course_c5 (tutor_id, course_name) VALUES ($1, $2)
            RETURNING course_id, tutor_id, course_name, posted_time",
            course.tutor_id,
            course.course_name,
        )
        .fetch_one(pool)
        .await?;

        return Ok(Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name,
            posted_time: row.posted_time,
        });
    }

    // NOTE: test conflict
    let result = query!(
        "INSERT INTO ezy_course_c5 (course_id, tutor_id, course_name) VALUES ($1, $2, $3)
        RETURNING course_id, tutor_id, course_name, posted_time",
        course.course_id,
        course.tutor_id,
        course.course_name,
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
            });
        }

        Err(e) => e,
    };

    //    if let sqlx::Error::Database(e) = &err {
    //        if e.code().unwrap() == "23505" {
    //            return Err(Error::AlreadyExists);
    //        }
    //    }

    dbg!(&err);

    if db_error_code(&err) == Some("23505".into()) {
        Err(Error::AlreadyExists)
    } else {
        Err(Error::DBError(err.to_string()))
    }
}

// TODO: sqlx::error::Error, sqlx::postgres::PgDatabaseError,
pub fn db_error_code(err: &SQLxError) -> Option<String> {
    let e2 = match err {
        sqlx::Error::Database(e) => e,
        _ => return None,
    };

    e2.code().map(|v| Some(v.into()))? // convert a Result to an option
}
