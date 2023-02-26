/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-24 18:59:35
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-26 22:00:28
 * @FilePath: \s1\httpserver\src\main.rs
 * @Description: 
 */
mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}
