// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The laugest number is {}", result);

    let mut number_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let mut integer = Point { x: 5, y: 10 };
    let float: Point<f32> = Point { x: 1.0, y: 4.0 };

    println!("{:?} {:?}", integer, float);
    println!("integer.x: {}", integer.x);

    println!("integer.x(): {}", integer.x());

    // let x = &mut number_list[0];
    // *x = 'a';
    // // number_list[0] = 'a';
    // println!("{}", number_list[0]);

    // let wont_work = Point { x: 5, y: 4.0 };
    float.distance_from_origin();
}
