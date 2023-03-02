/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-03-01 09:48:31
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-03-02 11:14:08
 * @FilePath: /ws/webservice/src/dbaccess/course.rs
 * @Description:
 */
use crate::errors::MyError;
use crate::models::course::Course;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where teacher_id = $1"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = rows
        .iter()
        .map(|r| Course {
            id: r.id,
            teacher_id: r.teacher_id.unwrap(),
            name: r.name.clone().unwrap(),
            time: Some(NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();

    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"select id, teacher_id, name, time from course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(row) = row {
        Ok(Course {
            id: row.id,
            teacher_id: row.teacher_id.unwrap(),
            name: row.name.clone().unwrap(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let row = sqlx
        ::query!(
            r#"insert into course (id, teacher_id, name) values($1, $2, $3) RETURNING id, teacher_id, name, time"#,
            new_course.id,
            new_course.teacher_id,
            new_course.name
        )
        .fetch_one(pool).await?;

    Ok(Course {
        id: row.id,
        teacher_id: row.teacher_id.unwrap(),
        name: row.name.clone().unwrap(),
        time: Some(NaiveDateTime::from(row.time.unwrap())),
    })
}
