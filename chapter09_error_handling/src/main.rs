use std::fs::File;
use std::io::{self, ErrorKind, Read};
fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    // v[99];
    // v.get(99)
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // let f = File::open("hello2.txt").unwrap();
    // println!("{}", f.);

    // let f = File::open("hello2.txt").expect("Failed to open hello.txt");

    let result = read_username_from_file();

    match result {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e),
    };
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("test.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("test.txt")?;
//     let mut s = String::new();

//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("test.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

use std::fs;
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("test.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
