// fn main() {
//     // let x = 5;
//     // let y = &x;

//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);



//     // let x = 5;
//     // let y = Box::new(x);

//     // assert_eq!(5, x);
//     // assert_eq!(5, *y);


//     use std::ops::Deref;

//     struct MyBox<T>(T);

//     impl<T> MyBox<T> {
//         fn new(x:T) -> MyBox<T> {
//             MyBox(x)
//         }
//     }

//     impl<T> Deref for MyBox<T> {
//         type Target = T;

//         fn deref(&self) -> &T {
//             &self.0
//         }
//     }

//     let x = 5;
//     let y = MyBox::new(x);
//     assert_eq!(5, x);
//     assert_eq!(5, *y); // *y <==> *(y.deref())
    
// }



use std::ops::Deref;

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {

    let m = MyBox::new(String::from("Rust"));

    // &m => &MyBox<String>
    // deref => &String
    // deref => &str
    hello(&m);
    hello(&(*m)[..]);

    hello("Rust");

}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}