/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-07 16:43:25
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-08 14:31:14
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
        .get("http://localhost:3000/teachers/")
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
        .render("teachers.html", &ctx)
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
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if params.name == ""  || params.image_url == "" || params.profile == "" {
        ctx.insert("error", "Params can't be empty!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_image_url", &params.image_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".to_string()));
    } else if params.name == "Dave" {
        ctx.insert("error", "Dave already exists!Params can't be empty!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_image_url", &params.image_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".to_string()));
    } else {
        let new_teacher = json!({
                "name": &params.name,
                "picture_url": &params.image_url,
                "profile": &params.profile
        });
        let awc_client = awc::Client::default();

        let res = awc_client
            .post("http://localhost:3000/teachers/")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;
        let teacher_response: TeacherResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
        s = Ok(format!("Congratulations! Your Id is:{}", teacher_response.id));
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s?))
}
