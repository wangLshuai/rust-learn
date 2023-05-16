enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Test(String),
}
use std::collections::HashMap;
fn main() {
    let mut v: Vec<i32> = Vec::new();

    // let v = Vec::<i32>::new();

    // let v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("Threr is no third element",),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[4];
    // v.push(6);

    // println!("{}", does_not_exist);
    let does_not_exist = v.get(4);

    // v.push(5);

    if let Some(i) = does_not_exist {
        println!("v[] is {}", i);
    } else {
        println!("V[] does not exist");
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        println!("{}", i);
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Test(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let r_2 = row.get(2);

    if let Some(i) = r_2 {
        if let SpreadsheetCell::Float(f) = i {
            println!("float: {}", f);
        }
    }

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    let s = "initial contents".to_string();

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s1 {}", s1);

    println!("{}", s);

    let s1 = String::from("hello");
    // let h = s1[0];

    let s1 = String::from("中国");
    println!("len of '{}' is {}", s1, s1.len());
    let s = &s1[0..3];
    println!("s: {}", s);

    for c in s1.chars() {
        println!("{}", c);
    }

    for c in "中国".chars() {
        println!("{}", c);
    }

    for b in "中国".bytes() {
        println!("{}", b);
    }
    println!("-----------------hashmap--------------------------");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{}", field_name);
    let score = scores.get("Blue");
    if let Some(s) = score {
        println!("Blue score: {}", s);
    }

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
