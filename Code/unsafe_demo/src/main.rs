// fn main() {
//     let mut num = 5;

//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;

//     // println!("r1: {}", *r1); // dereference of raw pointer is unsafe and requires unsafe function or block
//     // println!("r2: {}", *r2);
//     unsafe {
//         println!("r1: {}", *r1);
//         println!("r2: {}", *r2);
//     }

//     let address = 0x012345usize;
//     let r = address as *const i32;
//     unsafe {
//         println!("r: {}", *r); // error: process didn't exit successfully: `target\debug\unsafe_demo.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
//     }
// }



// unsafe fn dangerous() {

// }

// fn main() {
//     unsafe {
//         dangerous()
//     }
//     dangerous() // call to unsafe function is unsafe and requires unsafe function or block
// }



// use std::slice;

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     // let ptr = slice.len() as *mut i32;
//     let ptr = slice.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5, 6];

//     let r = &mut v[..];
//     let (a, b) = r.split_at_mut(3);
//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }



// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// fn main() {
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }



// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }

// fn main() {

// }



// static HELLO_WORLD: &str = "hello world!";

// fn main() {
//     println!("name is: {}", HELLO_WORLD);
// }



// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// fn main() {
//     add_to_count(3);

//     unsafe {
//         println!("COUNTER: {}", COUNTER);
//     }
// }



unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
}