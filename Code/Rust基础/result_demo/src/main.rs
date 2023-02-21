/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-01-06 14:39:27
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-01-12 15:25:16
 * @FilePath: /result_demo/src/main.rs
 * @Description:
 */
use std::{fs::File, io::{ErrorKind, self, Read}, error::Error};

fn main() {
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("error opening file: {:?}", error)
    //     }
    // };



    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:?}", e),
    //         },
    //         other_error => panic!("Error opening the file: {:?}", other_error),
    //     },
    // };



    
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Error creating file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Error opening file: {:?}", error);
    //     }
    // });



    // let f = File::open("hello.txt").unwrap();



    // let f = File::open("hello.txt").expect("无法打开文件 hello.txt");



    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("hello.txt");

    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut s = String::new();
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }

    // let reuslt = read_username_from_file();



    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut f = File::open("hello.txt")?;
    //     // EA -> EB, EA from -> EB
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    // }



    // 链式调用
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s)
    // }



    // let f = File::open("hello.txt")?;




}



// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;
//     Ok(())
// }