fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);

    // let v = vec![1, 2, 3, 4];
    // let third: &i32 = &v[2];
    // println!("The third element is: {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is: {}", third),
    //     None => println!("There is no third element"),
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);




    // let v = vec![1, 2, 3, 4, 5];
    // for i in &v {
    //     println!("{}", i);
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // for i in &mut v {
    //     *i += 50;
    // }
    // for i in &v {
    //     println!("{}", i);
    // }



    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(11.22),
        SpreadsheetCell::Text(String::from("bule")),
    ];
}
