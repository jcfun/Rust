// use std::time::SystemTime;
use std::time::Instant;

fn main() {
    // let start = SystemTime::now()
    //     .duration_since(SystemTime::UNIX_EPOCH)
    //     .expect("get millis error")
    //     .as_millis();
    let time = Instant::now();

    let num = fib(40);

    // let end = SystemTime::now()
    //     .duration_since(SystemTime::UNIX_EPOCH)
    //     .expect("get millis error")
    //     .as_millis();
    // println!("{} ms", end - start);
    println!("{:?} ms", time.elapsed());
    println!("{}", num);
}

fn fib(i: i32) -> i64 {
    if i <= 0 {
        return 0;
    }
    if i <= 2 {
        return 1;
    }
    fib(i - 1) + fib(i - 2)
}
