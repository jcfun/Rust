/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-08 21:01:49
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-08 21:15:39
 * @FilePath: /ws/wasm-client/src/errors.rs
 * @Description: 
 */
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MyError {
    SomeError(String),
}

impl From<String> for MyError {
    fn from(s: String) -> Self {
        MyError::SomeError(s)
    }
}

impl From<wasm_bindgen::JsValue> for MyError {
    fn from(js_value: wasm_bindgen::JsValue) -> Self {
        MyError::SomeError(js_value.as_string().unwrap())
    }
}