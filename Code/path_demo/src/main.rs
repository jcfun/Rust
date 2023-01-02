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

use rand::Rng;
use std::collections::HashMap;

use std::cmp::Ordering;
use std::io;
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
use std::io::{self, Write};

use std::collections::*;