use crate::{models::Course, state::AppState};
use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::{DateTime, Local, SecondsFormat, Utc};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let now: DateTime<_> = Local::now();
    let now = now.to_rfc3339_opts(SecondsFormat::Millis, true);

    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times\n", health_check_response, visit_count);
    *visit_count += 1;

    HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(("x-app-version", "0.1.0"))
        .insert_header(("date", now.as_str()))
        .body(response)
}

pub async fn new_course(app_state: web::Data<AppState>, course: web::Json<Course>) -> HttpResponse {
    println!("~~~ Received new course");

    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|v| v.tutor_id == course.tutor_id)
        .count() as i32;

    let course = Course {
        tutor_id: course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: course.course_name.clone(),
        post_time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(course.clone());
    // HttpResponse::Ok().json("Added course")
    HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> HttpResponse {
    let tutor_id: i32 = params.0;
    // HttpRequest.match_info().query("tutor_id").parse::<i32>().unwrap_or(0);

    let courses: Vec<Course> = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|v| v.tutor_id == tutor_id)
        .collect();

    if courses.len() > 0 {
        HttpResponse::Ok().json(courses)
    } else {
        HttpResponse::Ok().json("No courses found for tutor")
    }
}

pub async fn get_course_detail(
    app_data: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = (params.0, params.1);

    let result = app_data
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|v| v.tutor_id == tutor_id && v.course_id == Some(course_id))
        .ok_or("Course not found");
    // iter.find(is_true) reurn an option, ok_or convert None to Err(e)

    if let Ok(course) = result {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http::StatusCode, web};

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hell, this is test course".into(),
            course_id: None,
            post_time: None,
        });

        let app_state = web::Data::new(AppState::default());
        let resp = new_course(app_state, course).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_course_success() {
        let app_state = web::Data::new(AppState::default());

        let tutor_id = 1;
        let resp = get_courses_for_tutor(app_state, web::Path::from((tutor_id,))).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_once_course_detail() {
        let app_state: web::Data<AppState> = web::Data::new(AppState::default());

        let params = (1, 1);
        let resp = get_course_detail(app_state, web::Path::from(params)).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
