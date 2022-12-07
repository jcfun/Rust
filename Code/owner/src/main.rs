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

fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}