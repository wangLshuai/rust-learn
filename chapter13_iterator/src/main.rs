use std::vec;

fn main() {
    let mut v1 = vec![1, 2, 3];
    // let mut v1_iter = v1.iter_mut();
    let v1_iter = v1.iter();

    for val in v1_iter {
        // *val = 1;
        println!("Got {}", val);
    }

    // println!("{:?}", v1);

    // let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    {
        let iter2 = v1_iter.map(|x| *x + 1);

        // v1_iter.next();
        // let v1_1 = v1_iter.next().unwrap();
        // *v1_1 = 4;
        // for i in iter2 {
        //     println!("Got {}", i);
        // }

        // println!("{:?}", v1);

        let v2: Vec<_> = iter2.collect();
        println!("{:?}", v2);
    }

    println!("{:?}", v1);


    
}
