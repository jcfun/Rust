/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-10 16:59:31
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-10 16:59:46
 * @FilePath: /refCell_demo/src/lib.rs
 * @Description: 
 */
pub trait Messenger {
    fn send(&self, msg: &str);
}