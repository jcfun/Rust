// fn main() {
//     let favorite_color: Option<&str> = None;
//     let is_tuesday = false;
//     let age: Result<u8, _> = "34".parse();

//     if let Some(color) = favorite_color {
//         println!("Using your favorite color, {}, as the background", color);
//     } else if is_tuesday {
//         println!("Tuesday is green day!");
//     } else if let Ok(age) = age {
//         if age > 30 {
//             println!("Using purple as the background color");
//         } else {
//             println!("Using orange as the background color");
//         }
//     } else {
//         println!("Using blue as the background color");
//     }
// }



// fn main() {
//     let mut stack = Vec::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }



// fn main() {
//     let v = vec!['a', 'b', 'c'];

//     for (index, value) in v.iter().enumerate() {
//         println!("{} is at index {}", value, index);
//     }
// }



// fn main() {
//     let a = 5;

//     let (x, y, z) = (1, 2, 3);
// }



// fn foo(x: i32) {
//     // code goes here
// }

// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: ({}, {})", x, y);
// }
// fn main() {
//     let point = (3, 5);
//     print_coordinates(&point);
// }



// fn main() {
//     let a: Option<i32> = Some(5);
//     // let Some(x) = a; // error[E0005]: refutable pattern in local binding: `None` not covered
//     if let Some(x) = a {

//     }
//     if let x = 5 {} // warning: irrefutable `if let` pattern
// }



// fn main() {
//     let x = 1;

//     match x {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }
// }



fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}",  y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}