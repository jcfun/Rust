// pub trait Iterator {
//     type Item;
    
//     fn next(&mut self) -> Option<Self::Item>;
// }

// pub trait Iterator2<T> {
//     fn next(&mut self) -> Option<T>;
// }

// struct Counter {}

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// // impl Iterator for Counter { // conflicting implementations of trait `Iterator` for type `Counter`
// //     type Item = u32;

// //     fn next(&mut self) -> Option<Self::Item> {
// //         None
// //     }
// // }

// impl Iterator2<String> for Counter {
//     fn next(&mut self) -> Option<String> {
//         None
//     }
// }

// impl Iterator2<u32> for Counter {
//     fn next(&mut self) -> Option<u32> {
//         None
//     }
// }
    
// fn main() {
//     println!("Hello, world!");
// }



// use std::ops::Add;

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32, 
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + self.x,
//             y: self.y + self.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(Point{ x: 1, y: 0 } + Point { x: 2, y: 3 }, 
//         Point { x: 3, y: 3 });
// }



// use std::ops::Add;

// struct Millimeters(u32);
// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// fn main() {
// }



// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!")
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }

// fn main() {
//     let person = Human;
//     person.fly();
//     Pilot::fly(&person);
//     Wizard::fly(&person);
// }



// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from("Spot")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }

// fn main() {
//     println!("A baby dog is called a {}", Dog::baby_name());
//     println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
// }



// use std::fmt;

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl OutlinePrint for Point {} // `Point` doesn't implement `std::fmt::Display`

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// fn main() {
// }



use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}