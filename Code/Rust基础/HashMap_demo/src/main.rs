use std::num;

fn main() {
    use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);



    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let intial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    // println!("{:?}", scores);



    // let field_name = String::from("Favorite color");
    // let field_valule = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(&field_name, &field_valule);

    // println!("{}: {}", field_name, field_valule);



    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("blue");
    // let score = scores.get(&team_name);

    // match score {
    //     Some(s) => println!("{}", s),
    //     None => println!("team not exist"),
    // }



    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (k, v) in &scores {
    //     println!("{}: {}", k, v);
    // }



    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("blue"), 20);

    // println!("{:?}", scores);



    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // // scores.entry(String::from("yellow")).or_insert(50);
    // let e = scores.entry(String::from("yellow"));
    // println!("{:?}", e);
    // e.or_insert(50);

    // scores.entry(String::from("blue")).or_insert(50);
    // println!("{:?}", scores);



    // let text = "hello world wonderful world";
    // let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }
    // println!("{:#?}", map);



    // 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）
    let mut nums = vec![12, 23, 54, 21, 32, 8, 43, 23, 12, 45, 23];
    let middle_num = middle(&mut nums);
    println!("中位数为：{}", middle_num);
    let mode_num = mode(&mut nums);
    println!("众数为：{}", mode_num);

    fn middle(nums: &mut Vec<i32>) -> f64 {
        nums.sort();
        let length = nums.len();
        if length % 2 == 1 {
            return nums[length / 2] as f64;
        } else {
            return ((nums[length / 2 - 1] + nums[length / 2]) / 2) as f64;
        }
    }

    fn mode(nums: &mut Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for value in nums {
            let count = map.entry(value).or_insert(0);
            *count += 1;
        }
        let mut max_num: i32 = 0;
        let mut max_count: i32 = 0;
        for (num, count) in map {
            if count > max_count {
                max_count = count;
                max_num = *num;
            }
        }
        max_num
    }

}
