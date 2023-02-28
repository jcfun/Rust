use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

use super::models::Course;
use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // println!("Received new course");
    // let course_count = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == new_course.teacher_id)
    //     .collect::<Vec<Course>>()
    //     .len();
    // let new_course = Course {
    //     teacher_id: new_course.teacher_id,
    //     id: Some(course_count + 1),
    //     name: new_course.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };

    // app_state.courses.lock().unwrap().push(new_course);
    // HttpResponse::Ok().json("Course added")

    HttpResponse::Ok().json("Success")
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, )>,
) -> HttpResponse {
    // let teacher_id: usize = params.0;

    // let filtered_courses = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|course| course.teacher_id == teacher_id)
    //     .collect::<Vec<Course>>();

    // if filtered_courses.len() > 0 {
    //     HttpResponse::Ok().json(filtered_courses)
    // } else {
    //     HttpResponse::Ok().json("No courses found for teacher".to_string())
    // }

    HttpResponse::Ok().json("Success")
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    // let (teacher_id, course_id) = params.0;
    // let selected_course = app_state
    //     .courses
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
    //     .ok_or("Course not found");

    // match selected_course {
    //     Ok(course) => HttpResponse::Ok().json(course),
    //     Err(msg) => HttpResponse::Ok().json(msg)
    // }

    HttpResponse::Ok().json("Success")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        // let course = web::Json(Course {
        //     teacher_id: 1,
        //     name: "Test course".into(),
        //     id: None,
        //     time: None,
        // });
        // let app_state: web::Data<AppState> = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![]),
        // });
        // let resp = new_course(course, app_state).await;
        // assert_eq!(resp.status(), StatusCode::OK);

        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: None,
            time: None,
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        // let app_state: web::Data<AppState> = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![]),
        // });

        // let teacher_id: web::Path<(usize)> = web::Path::from((1));
        // let resp = get_courses_for_teacher(app_state, teacher_id).await;
        // assert_eq!(resp.status(), StatusCode::OK);

        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        // let app_state: web::Data<AppState> = web::Data::new(AppState {
        //     health_check_response: "".to_string(),
        //     visit_count: Mutex::new(0),
        //     courses: Mutex::new(vec![]),
        // });
        // let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        // let resp = get_course_detail(app_state, params).await;
        // assert_eq!(resp.status(), StatusCode::OK);

        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
