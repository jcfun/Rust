use crate::handlers::{get_all_teachers, show_register_form, handler_register};
use actix_files as fs;
use actix_web::web;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(web::resource("/").route(web::get().to(get_all_teachers)))
            .service(web::resource("/register").route(web::get().to(show_register_form)))
            .service(web::resource("/register-post").route(web::post().to(handler_register))),
    );
}