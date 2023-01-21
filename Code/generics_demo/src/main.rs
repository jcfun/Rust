fn main() {
    // fn largest(list: &[i32]) -> &i32 {
    //     let mut largest = &list[0];
    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }

    // let number_list = [34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);



    // fn largest<T>(list: &[T]) -> &T {
    //     let mut largest = &list[0];
    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }

    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);



    // struct Point<T> {
    //     x: T, 
    //     y: T,
    // }

    // let integer = Point{ x: 5, y: 10 };
    // let float = Point{ x: 1.0, y: 4.0};



    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }
    
    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
