/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-13 09:10:18
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-13 14:59:24
 * @FilePath: /circular_ref_demo/src/main.rs
 * @Description:
 */

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// fn main() {
    
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    
//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//     println!("a rc count after b creation = {}", Rc::strong_count(&b));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // it will overflow the stack
//     println!("a next item = {:?}", a.tail());

// }



use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong =  {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong =  {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong =  {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    }
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong =  {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

}