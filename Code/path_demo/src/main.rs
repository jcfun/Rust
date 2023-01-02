use std::fmt;
use std::io;

fn f1() -> fmt::Result {}

fn f2() -> io::Result {}



use std::fmt::Result as fmtResult;
use std::io::Result as ioResult;

fn f1() -> fmtResult {}

fn f2() -> ioResult {}



fn main() {
    println!("Hello, world!");
}
