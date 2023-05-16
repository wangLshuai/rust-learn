#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let penny = Coin::Penny;
    let pv = value_in_cents(penny);
    println!("penny value: {}", pv);

    let quarter_alaska = Coin::Quarter(UsState::Alaska);

    let qav = value_in_cents(quarter_alaska);

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("sevev"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;

    // match coin {
    //     Coin::Quarter(state) => {
    //         println!("State Quarter from {:?}", state)
    //     }
    //     _ => count += 1,
    // }

    count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State Quarter from {:?}", state)
    } else {
        count += 1;
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("i: {}", i);
            Some(i + 1)
        }
    }
}
