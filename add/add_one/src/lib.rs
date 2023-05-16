use rand::Rng;
pub fn add_one(x: i32) -> i32 {
    let rand: i32 = rand::thread_rng().gen_range(1, 101);
    println!("rand: {}", rand);
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
