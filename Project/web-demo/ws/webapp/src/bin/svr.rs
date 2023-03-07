/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-07 17:11:30
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-07 19:59:21
 * @FilePath: /ws/webapp/src/bin/svr.rs
 * @Description: 
 */
#[path = "../mod.rs"]
mod wa;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routers::app_config;
use std::env;
use wa::{errors, handlers, models, routers};

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not exist");
    println!("Listening on: {}", &host_port);

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new().app_data(web::Data::new(tera)).configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}