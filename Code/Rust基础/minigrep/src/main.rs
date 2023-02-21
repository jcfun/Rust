/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-01-26 08:49:54
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-01-27 21:36:38
 * @FilePath: /minigrep/src/main.rs
 * @Description: minigrep
 */

use std::env;
use std::process;
use minigrep::Config;


fn main() {

    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


