// fn main() {
//     let v = Some(0u8);
//     match v {
//         Some(3) => println!("three"),
//         _ => (),
//     }

//     if let Some(3) = v {
//         println!("three");
//     }
// }


fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}