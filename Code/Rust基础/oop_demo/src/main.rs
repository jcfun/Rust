/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-15 08:38:00
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-17 15:45:58
 * @FilePath: /oop_demo/src/main.rs
 * @Description: 
 */
// use oop_demo::{Draw, Screen, Button};

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         // 绘制一个选择框
//     }
// }

// fn main() {
    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("OK"),
    //         }),
    //     ],
    // };

    // screen.run();
// }



// use oop_demo::Post;

// fn main() {
//     let mut post = Post::new();

//     post.add_text("I ate a salad for lunch today");
//     assert_eq!("", post.content());

//     post.request_review();
//     assert_eq!("", post.content());

//     post.approve();
//     assert_eq!("I ate a salad for lunch today", post.content());

// }



use oop_demo::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

}