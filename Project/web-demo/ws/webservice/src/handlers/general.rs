/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-03-02 11:04:22
 * @LastEditors: error: error: git config user.name & please set dead value or install git && error: git config user.email & please set dead value or install git & please set dead value or install git
 * @LastEditTime: 2023-03-06 13:22:27
 * @FilePath: /ws/webservice/src/handlers/general.rs
 * @Description: 
 */
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    println!("health!");
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
