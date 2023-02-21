#![allow(unused)]
// enum IpAddrKind {
//     V4,
//     V6,
// }


// fn main() {
    
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(four);
//     route(six);
//     route(IpAddrKind::V6);

// }


// fn route(_ip_kind: IpAddrKind) {}


// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }


// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
    
//     let home = IpAddrKind::V4(127, 0, 0, 1);
//     let loopback = IpAddrKind::V6(String::from(":::1"));
    
// }


// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
    
//     let q = Message::Quit;
//     let m = Message::Move { x: 12, y: 24 };
//     let w = Message::Write(String::from("hello"));
//     let c = Message::ChangeColor(0, 255, 255);
    
// }


fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}