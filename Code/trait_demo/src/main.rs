use trait_demo::{Tweet, Summary};

/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-01-21 11:30:10
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-01-21 18:23:12
 * @FilePath: /trait_demo/src/main.rs
 * @Description: 
 */
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already konw, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize())
}
