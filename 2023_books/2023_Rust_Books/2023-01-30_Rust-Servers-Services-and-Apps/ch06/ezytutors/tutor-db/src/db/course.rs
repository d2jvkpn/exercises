#![allow(dead_code)]
use crate::{
    models::{Course, UpdateCourse},
    response::Error,
    utils,
};
use sqlx::{self, error::Error as SQLxError, postgres::PgPool, query_as};

pub async fn post_new_course(pool: &PgPool, course: Course) -> Result<Course, Error> {
    if course.tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    let err = match query_as!(
        Course,
        "INSERT INTO ezy_course_c6 (tutor_id, course_name, course_description,
          course_duration, course_level, course_format, course_language, course_structure,
          course_price)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *",
        course.tutor_id,
        course.course_name,
        course.course_description,
        course.course_duration,
        course.course_level,
        course.course_format,
        course.course_language,
        course.course_structure,
        course.course_price
    )
    .fetch_one(pool)
    .await
    {
        Ok(v) => return Ok(v),
        Err(e) => e,
    };
    dbg!(&err);

    //    if let sqlx::Error::Database(e) = &err {
    //        if e.code().unwrap() == "23505" {
    //            return Err(Error::AlreadyExists);
    //        }
    //    }

    if utils::db_error_code(&err) == Some("23505".into()) {
        Err(Error::AlreadyExists)
    } else {
        Err(Error::DBError(err))
    }
}

pub async fn get_courses_for_tutor(pool: &PgPool, tutor_id: i32) -> Result<Vec<Course>, Error> {
    if tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    let courses: Vec<Course> = query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1 ORDER BY course_id DESC",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

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
    if tutor_id <= 0 || course_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id or course_id".into()));
    }

    let course: Course = query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        // println!("!!! {:?}", e);
        match e {
            SQLxError::RowNotFound => Error::NotFound("course not exists".into()),
            _ => Error::DBError(e),
        }
    })?;

    Ok(course)
}

// Update course details
pub async fn update_course_details(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, Error> {
    if tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    // Retrieve current record
    let mut course = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course_c6 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| Error::NotFound("course id not found".into()))?;

    if !course.update(update_course) {
        return Err(Error::NoChanges);
    }

    sqlx::query_as!(
        Course,
        "UPDATE ezy_course_c6 SET course_name = $1, course_description = $2, course_format = $3, 
          course_structure = $4, course_duration = $5, course_price = $6, course_language = $7, 
          course_level = $8 WHERE tutor_id = $9 AND course_id = $10 RETURNING *",
        course.course_name,
        course.course_description,
        course.course_format,
        course.course_structure,
        course.course_duration,
        course.course_price,
        course.course_language,
        course.course_level,
        tutor_id,
        course_id,
    )
    .fetch_one(pool)
    .await
    .map(|v| Ok(v))?
}

pub async fn delete_course(pool: &PgPool, tutor_id: i32, course_id: i32) -> Result<(), Error> {
    if tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    // Prepare SQL statement
    let _ = sqlx::query!(
        "DELETE FROM ezy_course_c6 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {}
