/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-23 10:20:32
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-23 10:50:08
 * @FilePath: /s1/tcpclient/src/main.rs
 * @Description:
 */
use std::str;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
