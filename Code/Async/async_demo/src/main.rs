// use std::thread::sleep;
// use std::time::Duration;

// fn main() {
//     println!("Hello before reading file!");
//     let file1_contents = read_from_file1();
//     println!("{:?}", file1_contents);
//     println!("Hello after reading file1!");
//     let file2_contents = read_from_file2();
//     println!("{:?}", file2_contents);
//     println!("Hello after reading file2!");
// }

// fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     String::from("Hello, there from file 1")
// }

// fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     String::from("Hello, there from file 2")
// }



// use std::thread::{ self, sleep };
// use std::time::Duration;

// fn main() {
//     println!("Hello before reading file!");
//     let handle1 = thread::spawn(|| {
//         let file1_contents = read_from_file1();
//         println!("{:?}", file1_contents);
//     });
//     let handle2 = thread::spawn(|| {
//         let file2_contents = read_from_file2();
//         println!("{:?}", file2_contents);
//     });
//     handle1.join().unwrap();
//     handle2.join().unwrap();
// }

// fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     String::from("Hello, there from file 1")
// }

// fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     String::from("Hello, there from file 2")
// }



// use std::thread::sleep;
// use std::time::Duration;

// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
    
//     let h1 = tokio::spawn(async {
//         let _file1_contents = read_from_file1().await;
//     });
//     let h2 = tokio::spawn(async {
//         let _file2_contents = read_from_file2().await;
//     });
//     let _ = tokio::join!(h1, h2);
// }

// async fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     println!("{:?}", "Processing file 1");
//     String::from("Hello, there from file 1")
// }

// async fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     println!("{:?}", "Processing file 2");
//     String::from("Hello, there from file 2")
// }



// use std::thread::sleep;
// use std::time::Duration;

// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
    
//     let h1 = tokio::spawn(async {
//         let _file1_contents = read_from_file1().await;
//     });
//     let h2 = tokio::spawn(async {
//         let _file2_contents = read_from_file2().await;
//     });
//     let _ = tokio::join!(h1, h2);
// }

// use std::future::Future;

// fn read_from_file1() -> impl Future<Output = String> {
//     async  {
//         sleep(Duration::new(4, 0));
//         println!("{:?}", "Processing file 1");
//         String::from("Hello, there from file 1")
//     }
// }

// fn read_from_file2() -> impl Future<Output = String> {
//     async  {
//         sleep(Duration::new(3, 0));
//         println!("{:?}", "Processing file 2");
//         String::from("Hello, there from file 2")
//     }
// }



// use std::future::Future;
// use std::pin::Pin;
// use std::task::{ Context, Poll };
// use std::thread::sleep;
// use std::time::Duration;

// struct ReadFileFuture {}

// impl Future for ReadFileFuture {
//     type Output = String;

//     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
//         println!("Tokio! Stop polling me");
//         Poll::Pending
//     }
// }

// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
    
//     let h1 = tokio::spawn(async {
//         let future1 = ReadFileFuture {};
//         future1.await;
//     });

//     let h2 = tokio::spawn(async {
//         let file2_contents = read_from_file2().await;
//         println!("{:?}", file2_contents);
//     });
//     let _ = tokio::join!(h1, h2);
// }

// fn read_from_file2() -> impl Future<Output = String> {
//     async  {
//         sleep(Duration::new(2, 0));
//         println!("{:?}", "Processing file 2");
//         String::from("Hello, there from file 2")
//     }
// }



// use std::future::Future;
// use std::pin::Pin;
// use std::task::{ Context, Poll };
// use std::thread::sleep;
// use std::time::Duration;

// struct ReadFileFuture {}

// impl Future for ReadFileFuture {
//     type Output = String;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         println!("Tokio! Stop polling me");
//         cx.waker().wake_by_ref();
//         Poll::Pending
//     }
// }

// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
    
//     let h1 = tokio::spawn(async {
//         let future1 = ReadFileFuture {};
//         future1.await;
//     });

//     let h2 = tokio::spawn(async {
//         let file2_contents = read_from_file2().await;
//         println!("{:?}", file2_contents);
//     });
//     let _ = tokio::join!(h1, h2);
// }

// fn read_from_file2() -> impl Future<Output = String> {
//     async  {
//         sleep(Duration::new(2, 0));
//         println!("{:?}", "Processing file 2");
//         String::from("Hello, there from file 2")
//     }
// }



// use std::future::Future;
// use std::pin::Pin;
// use std::task::{ Context, Poll };
// use std::thread::sleep;
// use std::time::Duration;

// struct ReadFileFuture {}

// impl Future for ReadFileFuture {
//     type Output = String;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         println!("Tokio! Stop polling me");
//         cx.waker().wake_by_ref();
//         Poll::Ready(String::from("Hello, there from file 1"))
//     }
// }

// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
    
//     let h1 = tokio::spawn(async {
//         let future1 = ReadFileFuture {};
//         let file1_contents = future1.await;
//         println!("{:?}", file1_contents);
//     });

//     let h2 = tokio::spawn(async {
//         let file2_contents = read_from_file2().await;
//         println!("{:?}", file2_contents);
//     });
//     let _ = tokio::join!(h1, h2);
// }

// fn read_from_file2() -> impl Future<Output = String> {
//     async  {
//         sleep(Duration::new(2, 0));
//         String::from("Hello, there from file 2")
//     }
// }



use std::future::Future;
use std::pin::Pin;
use std::task::{ Context, Poll };
use std::thread::{ sleep, spawn };
use std::time::{ Duration, Instant };

struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("Hello, it's time for Future 1");
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            println!("Hello, it's not yet time for Future 1. Going to sleep");
            let waker = cx.waker().clone();
            let expiration_time = self.expiration_time;
            spawn(move || {
                let current_time = Instant::now();
                if current_time < expiration_time {
                    sleep(expiration_time - current_time);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(async {
        let future1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000),
        };
        println!("{:?}", future1.await);
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        println!("{:?}", file2_contents);
    });
    let _ = tokio::join!(h1, h2);
}

fn read_from_file2() -> impl Future<Output = String> {
    async  {
        sleep(Duration::new(2, 0));
        String::from("Future2 has completed")
    }
}