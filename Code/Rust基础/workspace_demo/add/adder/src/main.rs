/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-08 15:06:31
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-08 15:24:09
 * @FilePath: /add/adder/src/main.rs
 * @Description:
 */
use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    )
}
