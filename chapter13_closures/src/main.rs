use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Nest, do {} situps!", expensive_closure.value(intensity))
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run fo {} minutes",
                expensive_closure.value(intensity)
            )
        }
    }
}

// 闭包和函数
fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn test() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    let x = add_one_v3(2);
    let x = add_one_v4(2);

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_spcified_value = 6;
    let simulated_random_number = 7;

    generate_workout(simulated_user_spcified_value, simulated_random_number);

    let x = 4;
    let equal_to_x = |z| z == x;

    println!("{}", x);
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    let equal_to_x = |z: &[i32]| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(&y));

    assert!(equal_to_x(&y));
    println!("can't use x here: {:?}", x);
}
