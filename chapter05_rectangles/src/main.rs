#[derive(Debug)]
struct Reactangle {
    width: u32,
    height: u32,
}
impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Reactangle) -> bool {
        if self.height > rect.height && self.width > rect.width {
            return true;
        }
        return false;
    }
    fn square(size: u32) -> Reactangle {
        Reactangle {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Reactangle {
        width: 30,
        height: 50,
    };
    let rect2 = Reactangle {
        width: 10,
        height: 40,
    };
    let rect3 = Reactangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect1.can_hold(&rect3));

    print!("rect1:{:?}", rect1);
    println!(
        "The area of the reactangle is {} square pixels.",
        rect1.area()
    );

    let square = Reactangle::square(30);

    println!("square(30).area: {}", square.area());
}
