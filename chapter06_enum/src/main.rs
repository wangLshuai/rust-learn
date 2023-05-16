#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // println!("four:{:#?}, six: {:#?}", four, six);
    // route(four);
    let home = IpAddrKind::V4(127, 0, 0, 1);
    // println!("{}", Some(home));

    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

fn route(ip_type: IpAddrKind) {}
