fn main() {
    let x = 5;
    let x = x + 1;
    println!("The value of x is:{}", x);

    let s = "中国";
    println!("中国.len():{}", s.len());

    let _guess: u32 = "42".parse().expect("Not a number!");

    let c: i8 = 127;

    println!("c:{}", c);

    let _x = 2.0;

    let _y: f32 = 3.0;

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _remainder = 43 % 5;

    let _t = true;
    let _f: bool = false;

    let _z = ' ';
    let _heart_eyed_cat = ' ';

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = _tup;

    println!("The value of y is:{}", y);
    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _b = [3; 5];

    let _first = _a[0];
    let _second = _a[1];

    let index = 4;

    let element = _a[index];

    println!("The value of element is:{}", element);
    // Hello,world
    // So we're doing something complicated here,long enough that we need
    // multiple lines of comments to do int! Whew! Hopefully, this comment will
    // explain what's going on.
    println!("Hello,world"); // I'm feeling lucky today.

    another_function(5, 6);

    let _x = 5;
    let _y = {
        let x = 3;
        x + 1
    };

    let x = five();
    println!("The value of x is{}", x);

    let x = plus_one(5);
    println!("The value of x is {}", x);

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println! {"number is not divissible by 4,3,or 2"};
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // loop {
    //     println! {"again!"};
    // }

    let mut conter = 0;
    let result = loop {
        conter += 1;

        if conter == 10 {
            break conter * 2;
        };
    };
    println!("The value of result: {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFOFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println! {"斐波那契"};
    feb(10);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is:{}", x);
    println!("The value of y is:{}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn feb(level: u32) {
    let mut a1 = 0;
    let mut a2 = 1;

    for index in (1..level + 1) {
        let tmp = a2;
        a2 = a1 + a2;
        a1 = tmp;
        println!("value: {}", a2);
    }
}
