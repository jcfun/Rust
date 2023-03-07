/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-07 16:43:25
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-07 19:50:12
 * @FilePath: /ws/webapp/src/handlers.rs
 * @Description: 
 */
use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, MyError> {
    let awc_client = awc::Client::default();

    let res = awc_client
        .get("http://localhost:3000/teachers")
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>()
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    let s = tmpl
        .render("teacher.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()));
    Ok(HttpResponse::Ok().content_type("text/html").body(s?))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, MyError> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_image_url", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()));
    Ok(HttpResponse::Ok().content_type("text/html").body(s?))
}


pub async fn handler_register(
    _tmpl: web::Data<tera::Tera>,
    _params: web::Form<TeacherRegisterForm>
) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().finish())
}