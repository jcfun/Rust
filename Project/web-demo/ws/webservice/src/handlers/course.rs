use crate::{
    db_access::course::*,
    errors::MyError,
    state::AppState, models::course::{Course, CreateCourse, UpdateCourse},
};

use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    
    post_new_course_db(&app_state.db, new_course.try_into()?).await.map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    web::Path(teacher_id): web::Path<(i32)>,
) -> Result<HttpResponse, MyError> {
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    web::Path((teacher_id, course_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    web::Path((teacher_id, course_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    web::Path((teacher_id, course_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course)
    .await
    .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[ignore]
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
            id: Some(6),
            time: None,
        });
        let resp = new_course(course, app_state).await.unwrap();
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
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
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
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
