/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-28 12:56:05
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-06 16:55:37
 * @FilePath: /ws/webservice/src/bin/teacher-service.rs
 * @Description:
 */
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;
use routers::*;
use state::AppState;
use crate::errors::MyError;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../models/mod.rs"]
mod models;

#[path = "../dbaccess/mod.rs"]
mod db_access;

#[path = "../errors.rs"]
mod errors;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I' m OK.".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };
    println!("正在监听3000端口...");
    HttpServer::new(app).bind("::0:3000")?.run().await
}
