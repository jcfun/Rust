/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-07 16:29:47
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-08 12:55:36
 * @FilePath: /ws/webapp/src/routers.rs
 * @Description:
 */
use crate::handlers::{get_all_teachers, handler_register, show_register_form};
use actix_files as fs;
use actix_web::web;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./webapp/static").show_files_listing())
            .service(web::resource("/").route(web::get().to(get_all_teachers)))
            .service(web::resource("/register").route(web::get().to(show_register_form)))
            .service(web::resource("/register-post").route(web::post().to(handler_register))),
    );
}
