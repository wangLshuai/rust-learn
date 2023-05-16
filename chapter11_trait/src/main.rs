use core::prelude;

use chapter11_trait::aggregator::{NewsArticle, Summary, Tweet};

// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable(b: bool) -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaly already know, people"),
        reply: false,
        retweet: false,
    }
}
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl<T> Display for Pair<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let newsarticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh,PA,USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New argicle available! {}", newsarticle.summarize());

    notify(&tweet);
    notify(&newsarticle);

    let s = 3.to_string();

    let s = Pair::new(3, 4);
    println!("{}", s);
}
