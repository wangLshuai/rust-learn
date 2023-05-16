fn main() {
    let a: usize = 9999999999;
    let mut sum: usize = 0;

    for i in 0..=a {
        if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            sum += i;
        } else {
            sum -= i;
        }
    }
    println!("sum = {}", sum);

    // let mut c: u8 = 0;
    // c = c + 8;
    // println!("{}", c);
    // c = c + 128;
    // println!("{}", c);
    // c = c + 128;
    // println!("{}", c);
}
