/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-03-02 11:04:22
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-03-02 11:05:34
 * @FilePath: /ws/webservice/src/handles/general.rs
 * @Description: 
 */
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
