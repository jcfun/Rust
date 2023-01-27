/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-01-24 21:53:06
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-01-24 22:04:11
 * @FilePath: /test_demo_lib/tests/another_integration_tests.rs
 * @Description: 
 */
use test_demo_lib;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_demo_lib::add_two(2));
}