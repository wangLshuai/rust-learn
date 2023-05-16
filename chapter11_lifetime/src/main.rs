struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn main() {
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2, "hello");
    println!("The lognest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xya");
        let result = longest(string1.as_str(), string2.as_str(), "hello");
        println!("The longest string is {}", result);
    }

    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str(), "helllo");
    }
    // println!("The longest string is {}", result);
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetime generics
    let s: &'static str = "I have a static lifetime.";
}

use std::fmt::Display;
fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn test<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
