/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-06 15:11:14
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-06 15:40:13
 * @FilePath: /ws/webservice/src/models/teacher.rs
 * @Description:
 */

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32, // serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: web::Json<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher {
    fn from(update_teacher: web::Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        }
    }
}
