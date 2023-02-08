use crate::{
    models::{Tutor, UpdateTutor},
    response::Error,
    utils,
};
use sqlx::{error::Error as SQLxError, postgres::PgPool, query_as};

pub async fn post_new_tutor(pool: &PgPool, tutor: Tutor) -> Result<Tutor, Error> {
    let err = match query_as!(
        Tutor,
        "INSERT INTO ezy_tutor_c6 (tutor_name, tutor_pic_url, tutor_profile)
		  VALUES ($1, $2, $3)
		RETURNING *",
        tutor.tutor_name,
        tutor.tutor_pic_url,
        tutor.tutor_profile,
    )
    .fetch_one(pool)
    .await
    {
        Ok(v) => return Ok(v),
        Err(e) => e,
    };

    if utils::db_error_code(&err) == Some("23505".into()) {
        Err(Error::AlreadyExists)
    } else {
        Err(Error::DBError(err))
    }
}

pub async fn get_all_tutors(pool: &PgPool) -> Result<Vec<Tutor>, Error> {
    let tutors: Vec<Tutor> = query_as!(Tutor, "SELECT * FROM ezy_tutor_c6 ORDER BY tutor_id DESC")
        .fetch_all(pool)
        .await?;

    match tutors.len() {
        0 => Err(Error::NotFound("tutors not found for tutor".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details(pool: &PgPool, tutor_id: i32) -> Result<Tutor, Error> {
    if tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    let tutor: Tutor = query_as!(Tutor, "SELECT * FROM ezy_tutor_c6 WHERE tutor_id = $1", tutor_id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            SQLxError::RowNotFound => Error::NotFound("tutor not exists".into()),
            _ => Error::DBError(e),
        })?;

    Ok(tutor)
}

pub async fn update_tutor_details(
    pool: &PgPool,
    tutor_id: i32,
    update_tutor: UpdateTutor,
) -> Result<Tutor, Error> {
    if tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    let mut tutor = query_as!(Tutor, "SELECT * FROM ezy_tutor_c6 WHERE tutor_id = $1", tutor_id,)
        .fetch_one(pool)
        .await
        .map_err(|_err| Error::NotFound("tutor id not found".into()))?;

    if !tutor.update(update_tutor) {
        return Err(Error::NoChanges);
    }

    query_as!(
        Tutor,
        "UPDATE ezy_tutor_c6 SET tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3
		WHERE tutor_id = $4
		RETURNING *",
        tutor.tutor_name,
        tutor.tutor_pic_url,
        tutor.tutor_profile,
        tutor_id,
    )
    .fetch_one(pool)
    .await
    .map(|v| Ok(v))?
}

pub async fn delete_tutor(pool: &PgPool, tutor_id: i32) -> Result<(), Error> {
    if tutor_id <= 0 {
        return Err(Error::InvalidArgument("invalid tutor_id".into()));
    }

    // Prepare SQL statement
    let _ = sqlx::query!("DELETE FROM ezy_tutor_c6 WHERE tutor_id = $1", tutor_id,)
        .execute(pool)
        .await?;
    Ok(())
}
