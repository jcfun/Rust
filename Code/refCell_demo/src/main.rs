/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-10 19:59:02
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-13 00:00:52
 * @FilePath: \refCell_demo\src\main.rs
 * @Description: 
 */
// fn main() {
//     let x = 5;
//     // let y = &mut x;
// }

use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
}