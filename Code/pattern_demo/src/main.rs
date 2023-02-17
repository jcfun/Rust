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



// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {:?}",  y),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {:?}", x, y);
// }



// fn main() {
//     let x = 1;
//     let y = Some(1);

//     match x {
//         1 | 2 => println!("one or two"),
//         3 => println!("three"),
//         _ => println!("anything"),
//     }

//     match y {
//         Some(1 | 2) => println!("one or two"),
//         Some(3) => println!("three"),
//         _ => println!("anything"),
//     }
// }



// fn main() {
//     let x = 5;
//     match x {
//         1 ..= 5 => println!("one through five"),
//         _ => println!("something else"),
//     }

//     let x = 'c';
//     match x {
//         'a' ..= 'j' => println!("early ASCII letter"),
//         'k' ..= 'z' => println!("late ASCII letter"),
//         _ => println!("something else"),
//     }
// }



// struct Point {
//     x: i32,
//     y: i32,
// }
// fn main() {
//     let p = Point { x: 0, y: 7 };

//     let Point { x: a, y: b } = p;
//     assert_eq!(0, a);
//     assert_eq!(7, b);

//     let Point { x, y } = p;
//     assert_eq!(0, x);
//     assert_eq!(7, y);

//     match p {
//         Point { x, y: 0} => println!("On the x axis at {}", x),
//         Point { x: 0, .. } => println!("other"),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }



// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msg = Message::ChangeColor(0, 160, 255);

//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure")
//         }
//         Message::Move { x, y } => {
//             println!("Move in the x direction {} and in the y direction {}", x, y)
//         }
//         Message::Write(text) => println!("Text message: {}", text),
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {}, green {}, and blue {}", r, g, b)
//         }
//     }
// }



// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(Color),
// }

// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("Change the color to red {}, green {}, and blue {}", r, g, b)
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
//         }
//         _ => (),
//     }
// }



// struct Point {
//     x: i32, 
//     y: i32,
// }

// fn main() {
//     let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
// }



// fn foo(_: i32, y: i32) {
//     println!("This code only uses the y parameter: {}", y);
// }

// fn main() {
//     foo(3, 4);
// }



// fn main() {
//     let mut setting_value = Some(5);
//     let new_setting_value = Some(10);

//     match (setting_value, new_setting_value) {
//         (Some(_), Some(_)) => {
//             println!("Can't overwrite an existing customized value");
//         }
//         _ => {
//             setting_value = new_setting_value;
//         }
//     }
//     println!("setting is {:?}", setting_value);

//     let numbers = (2, 4, 8, 16, 32);

//     match numbers {
//         (first, _, third, _, fifth) => {
//             println!("Some numbers: {}, {}, {}", first, third, fifth)
//         },
//     }
// }



// fn main() {
//     let _x = 5;
//     let y = 10; // if this is intentional, prefix it with an underscore: `_y`

//     let s = Some(String::from("Hello!"));

//     if let Some(_s) = s {
//         println!("found a string");
//     }

//     println!("{:?}", s); // borrow of partially moved value: `s`


//     let s = Some(String::from("Hello!"));

//     if let Some(_) = s {
//         println!("found a string");
//     }

//     println!("{:?}", s);
// }



// struct Point {
//     x: i32,
//     y: i32,
//     z: i32,
// }

// fn main() {
//     let origin = Point { x: 0, y: 0, z: 0 };

//     match origin {
//         Point { x, .. } => println!("x is {}", x),
//     }

//     let numbers = (2, 4, 8, 16, 32);

//     match numbers {
//         (first, .., last) => {
//             println!("Some numbers: {}, {}", first, last);
//         },
//     }

//     match numbers {
//         (.., second, ..) => { // can only be used once per tuple pattern
//             println!("Some numbers: {}", second,)
//         }
//     }
// }



// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x < 5 => println!("less than five: {}", x),
//         Some(x) => println!("{}", x),
//         None => (),
//     }
// }

// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("Matched, n = {:?}", n),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {:?}", x, y);
// }

// fn main() {
//     let x = 4;
//     let y = false;

//     match x {
//         4 | 5 | 6 if y => println!("yes"),
//         _ => println!("no"),
//     }
// }



enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}