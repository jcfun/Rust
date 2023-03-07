/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-07 09:13:14
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-07 21:19:13
 * @FilePath: /ws/webapp/src/model.rs
 * @Description: 
 */
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub image_url: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}