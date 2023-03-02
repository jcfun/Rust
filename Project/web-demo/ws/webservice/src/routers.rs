/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-28 12:46:54
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-03-02 11:16:39
 * @FilePath: /ws/webservice/src/routers.rs
 * @Description:
 */
use crate::handlers::{course::*, general::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail)),
    );
}
