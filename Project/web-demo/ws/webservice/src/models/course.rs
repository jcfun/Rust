/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-28 12:46:48
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-03-02 22:06:42
 * @FilePath: /ws/webservice/src/models/course.rs
 * @Description: 
 */
use actix_web::web;
use chrono::NaiveDateTime;
// use openssl::derive;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug,Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(course: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}
