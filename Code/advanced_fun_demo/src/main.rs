/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-02-20 15:34:06
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-02-20 17:02:57
 * @FilePath: /advanced_fun_demo/src/main.rs
 * @Description: 
 */
// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);

//     println!("The answer is: {}", answer);
// }



fn main() {
    // let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> = list_of_numbers
    //     .iter()
    //     .map(|i| i.to_string())
    //     .collect();

    // // 我们也可以使用一个函数作为map的参数，如下所示：

    // let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> = list_of_numbers
    //     .iter()
    //     .map(ToString::to_string)
    //     .collect();


    // #[derive(Debug)]
    // enum Status {
    //     Value(u32),
    //     Stop,
    // }
    
    // let list_of_statuses: Vec<Status> =
    //     (0u32..20)
    //     .map(|x| Status::Value(x))
    //     .collect();

    // let list_of_statuses: Vec<Status> =
    //     (0u32..20)
    //     .map(Status::Value)
    //     .collect();

    // println!("{:#?}", list_of_statuses)
}



// fn returns_closure() -> Fn(i32) -> i32 { // does not have a constant size known at compile-time, trait objects must include the `dyn` keyword
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}  