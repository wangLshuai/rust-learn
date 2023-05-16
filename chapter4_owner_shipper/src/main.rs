fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);

    let s = String::from("hello");
    // takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();

    println!("s1:{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s3:{}", s3);

    let l = calculate_length(&s3);

    println!("{} len:{}", s3, l);

    let mut s = String::from("hello");

    let r1 = &mut s;

    let r2 = &mut s;

    change(&mut s);

    change(&mut s);

    println!("{}", s);

    println!("===========================================");

    let mut s = String::from("hello");

    let _r1 = &s;

    let _r2 = &s;

    println!("_r1:{}", _r1);

    let r3 = &mut s;

    r3.push_str("h");

    println!("s: {}", s);

    let reference_to_nothing = danagle();

    let mut s = String::from("hello world");
    let fw = first_word(&s);

    // s.clear();
    println!("{}", fw);

    let hello = &s[..5];
    let world = &s[6..s.len()];

    println!("{}", world);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn takes_ownership(some_string: String) {
    println!("\"{}\"", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integeer: i32) {
    println!("{}", some_integeer);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn danagle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut si = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            si = i;
            continue;
        }
        if item == b' ' {
            return &s[si..i];
        }
    }

    return &s[..];
}
