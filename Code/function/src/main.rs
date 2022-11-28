fn main() {
    println!("Hello, world!");
    another_function(5, 6);
    // let x = (let y = 6); // expected expression, found statement (`let`)

    let y = {
        let x = 1;
        x + 3
    };


    let x = five(6);
    println!("The value of x is {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("Another function!");
    println!("the value of x and y is: {}, {}", x, y);
}

fn five(x: i32) -> i32 {
    5 + x
}