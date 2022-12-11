#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, ohter: &Rectangle) -> bool {
        self.width > ohter.width && self.height > ohter.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let w = 30;
    // let h = 50;
    // println!("{}", area(w, h));

    // let rect = (30, 50);
    // println!("{}", area(rect));

    let s = Rectangle::square(20);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        height: 55,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    // println!("{:#?}", rect);
}

// fn area(widht: u32, height: u32) -> u32 {
//     widht * height
// }

// fn area(dim: (u32, u32)) -> u32 {
//     dim.0 * dim.1
// }

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
