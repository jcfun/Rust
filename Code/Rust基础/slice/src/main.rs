// fn main() {
//     let mut s = String::from("hello world");
//     let word_index = first_world(&s);
//     s.clear();
//     println!("{}", word_index)
// }

// fn first_world(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }


// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];

//     let hello1 = &s[..5];
//     let world1 = &s[6..];

//     let whole = &s[0..s.len()];
//     let whole1 = &s[..];

//     println!("{} {}, {} {}", hello, world, hello1, world1);
//     println!("{}, {}", whole, whole1);

// }

// fn main() { 

//     let mut s = String::from("hello world");
//     let word_index = first_word(&s);

//     // s.clear();
//     println!("{}", word_index);
// }

// fn first_word(s: &String) -> &str {

//     let bytes = s.as_bytes();

//     for(i, &item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn main() {

//     let s = "hello world";

//     println!("{}", s);

// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}
