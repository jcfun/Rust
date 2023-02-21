// fn main() {
//     // s 不可用
//     let s = "hello"; // s 可用
//     // 可以对 s 进行相关的操作
// } // 作用域到此结束，s 不再可用

// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world");

//     println!("{}", s);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn main() {
//     let x = 5;
//     let y = x;

//     println!("x = {}, y = {}", x, y);
// }

// fn main() {
//     let s = String::from("hello");

//     take_ownership(s);

//     let x = 5;

//     makes_copy(x);

//     println!("x: {}", x);

// }
// fn take_ownership(some_string: String) {
//     println!("{}", some_string);
// }
// fn makes_copy(some_number: i32) {
//     println!("{}", some_number);
// }

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership 将返回值
//                                     // 转移给 s1

//     let s2 = String::from("hello"); // s2 进入作用域

//     let s3 = takes_and_gives_back(s2); // s2 被移动到
//                                        // takes_and_gives_back 中,
//                                        // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 离开作用域并被丢弃

// fn gives_ownership() -> String {
//     // gives_ownership 会将
//     // 返回值移动给
//     // 调用它的函数

//     let some_string = String::from("yours"); // some_string 进入作用域.

//     some_string // 返回 some_string
//                 // 并移出给调用的函数
//                 //
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域
//     //

//     a_string // 返回 a_string 并移出给调用的函数
// }


// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() 返回字符串的长度

//     (s, length)
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

//     let r2 = &mut s;
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     let r3 = &mut s; // 大问题

//     println!("{}, {}, and {}", r1, r2, r3);
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
