// const MAX_POINTS: u32 = 100_000;

fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum number of {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is {}", y);

    let spaces = "        ";
    let spaces = spaces.len();
    println!("{}", spaces);


    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    let x = 2.0;
    let y: f32 = 3.0;


    let sum = 5 + 10;
    let difference = 95.5  - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 54 % 5;


    let t = true;
    let f: bool = false;


    let x = 'x';
    let y: char = '卍';
    let z = '😂';


    let tup: (i32, f64, u8) = (500, 6.3, 1);
    
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);


    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];

    let first = a[0];
    let second = a[1];
    let third = a[27]; // index out of bounds


}
