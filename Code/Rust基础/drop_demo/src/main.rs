/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-10 10:25:40
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-10 11:19:13
 * @FilePath: /drop_demo/src/main.rs
 * @Description:
 */

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with date `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
}
