/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-20 08:51:45
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-20 11:27:46
 * @FilePath: /advanced_type_demo/src/main.rs
 * @Description: 
 */
// type Kilometers = i32;

// fn main() {
//     let x: i32 = 5;
//     let y: Kilometers = 5;
//     println!("x + y = {}", x + y);
// }



// type Thunk = Box<dyn Fn() + Send + 'static>;

// fn take_long_type(f: Thunk) {
//     // --snip--
// }

// fn returns_long_type() -> Thunk {
//     Box::new(|| println!("Hi"))
// }

// fn main() {
//     let f: Thunk = Box::new(|| println!("Hi"));
// }



// // use std::io::Error;
// // type Result<T> = Result<T, std::io::Error>
// use std::fmt;

// type Result<T> = std::io::Result<T>;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
// }

// fn main() {
// }



// fn bar() -> ! { // mismatched types, expected `!`, found `()`

// }

// fn main() {
//     let guess = "";

//     loop {
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//     }
// }



// fn main() {
//     print!("forever ");

//     loop {
//         print!("and ever ");
//     }
// }



// fn main() {
//     let s1: str = "Hello there!";
//     let s2: str = "How's it going?";
// }



// fn generic<T>(t: T) {
//     // --略
// }

// fn generic<T: Sized>(t: T) {
//     // --略
// }