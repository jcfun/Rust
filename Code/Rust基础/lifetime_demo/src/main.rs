/*
 * @Author: jc-fun urainstar@gmail.com
 * @Date: 2023-01-22 16:11:45
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-01-22 21:19:20
 * @FilePath: /lifetime_demo/src/main.rs
 * @Description: 生命周期
 */
fn main() {
    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r);
    // }



    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);


    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }


    
    
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    
        


    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     let result = String::from("abc");
    //     result.as_str()
    // }



    
    // struct ImportantExcerpt<'a> {
    //     part: &'a str,
    // }

    // let novel = String::from("Call me Ishmael. Some years age ...");

    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // let i = ImportantExcerpt {
    //     part: first_sentence
    // };




    // fn first_word(s: &str) -> &str {
    //     let bytes = s.as_bytes();
    
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return &s[0..i];
    //         }
    //     }
    
    //     &s[..]
    // }




    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }


}
