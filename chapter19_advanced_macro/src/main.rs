#[macro_export]
macro_rules! vec {
    ($( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*println!("hello");temp_vec
        }
    };
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // let v = vec![1, 2, 3];
    // println!("{:?}", v);

    // Pancakes::hello_macro();
    let num: u64 = String::from("10000000000").parse().unwrap();
    let mut i = 0;

    loop {
        i = i + 1;
        if i > num {
            break;
        }
    }
    println!("{}", i);
}
