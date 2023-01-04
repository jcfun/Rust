fn main() {
    
    // let mut s = String::new();



    // let data = "initial contents";
    // let s = data.to_string();

    // let s1 = "initial contents".to_string();



    // let s2 = String::from("initial contents");



    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");



    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!');
    // println!("{}", s); 



    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world");
    // let s3 = s1 + &s2;
    // println!("{}", s3);
    // println!("{}", s1);
    // println!("{}", s2);



    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // // let s3 = s1 + "-" + &s2 + "-" + &s3;
    // // println!("{}", s3);

    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);


    // let s1 = String::from("hello");
    // let h = s1[0];


    // let len = String::from("Hola").len();
    // println!("{}", len);

    // let len = String::from("Здравствуйте").len();
    // println!("{}", len);

    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // // З: 208 151

    

    let w = "नमस्ते"; // 梵文书写的印度语单词
    for b in w.bytes() { // 字节
        println!("{}", b);
    }

    for b in w.chars() { // 标量值
        println!("{}", b);
    }

    


}
