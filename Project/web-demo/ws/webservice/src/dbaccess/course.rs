/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-03-01 09:48:31
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-06 14:43:29
 * @FilePath: /ws/webservice/src/dbaccess/course.rs
 * @Description:
 */
use crate::errors::MyError;
use crate::models::course::{Course, UpdateCourse, CreateCourse};
use sqlx::postgres::PgPool;  

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"select * from course where teacher_id = $1"#,
        teacher_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"select * from course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id,
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: CreateCourse) -> Result<Course, MyError> {
    let row = sqlx
        ::query_as!(
            Course,
            r#"insert into course (teacher_id, name, description, format, structure, duration, price, language, level) values($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
            new_course.teacher_id,
            new_course.name, 
            new_course.description,
            new_course.format,
            new_course.structure,
            new_course.duration,
            new_course.price,
            new_course.language,
            new_course.level,
        )
        .fetch_one(pool).await?;

    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "delete from course where teacher_id = $1 and id = $2",
        teacher_id,
        id,
    ).execute(pool)
    .await?;

    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "select * from course where teacher_id = $1 and id = $2",
        teacher_id,
        id
    ).fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name = if let Some(name) = update_course.name {
        Some(name)
    } else {
        current_course_row.name
    };
    let description = if let Some(description) = update_course.description {
        Some(description)
    } else {
        current_course_row.description
    };
    let format = if let Some(format) = update_course.format {
        Some(format)
    } else {
        current_course_row.format
    };
    let structure = if let Some(structure) = update_course.structure {
        Some(structure)
    } else {
        current_course_row.structure
    };
    let duration = if let Some(duration) = update_course.duration {
        Some(duration)
    } else {
        current_course_row.duration
    };
    let price = if let Some(price) = update_course.price {
        Some(price)
    } else {
        current_course_row.price
    };
    let language = if let Some(language) = update_course.language {
        Some(language)
    } else {
        current_course_row.language
    };
    let level = if let Some(level) = update_course.level {
        Some(level)
    } else {
        current_course_row.level
    };

    let course_row = sqlx::query_as!(
        Course,
        "update course set name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7,
        level = $8 where teacher_id = $9 and id = $10 RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level",
        name, description, format, structure, duration, price, language, level, teacher_id, id
    ).fetch_one(pool)
    .await;

    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }

}