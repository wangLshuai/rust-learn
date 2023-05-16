use std::fmt;
use std::io::Error;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }
fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    f();

    let s = String::from("hello");

    let s: &str = "hello there";
    println!("{}", s);

    let v = vec![1, 2, 3, 4, 5];
    let vs = &v[1..2];

    loop {
        let guess = String::from("123");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("{}", guess);
        break;
    }
}
