#[derive(Debug)]
struct Test {
    id: u32,
}

impl Test {
    fn add(&mut self) {
        self.id = self.id + 1;
    }
    fn getOwn(self) {
        println!("getOwn id:{}", self.id);
    }
}
impl Drop for Test {
    fn drop(&mut self) {
        println!("id address:{:p},id:{}", &self.id, self.id);
    }
}
fn main() {
    let t = Test { id: 2 };

    println!("id address:{:p}", &t.id);

    let mut t2 = Test { id: 3 };
    t2.add();
    // t2.getOwn();
    println!("-----------------");

    let mut v = vec![
        Some(Test { id: 1 }),
        Some(Test { id: 2 }),
        Some(Test { id: 3 }),
    ];

    println!("{}", v[0].as_ref().unwrap().id);

    let t = v[0].take();

    println!("{:?}", v[0]);
}
