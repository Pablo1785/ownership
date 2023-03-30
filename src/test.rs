struct A {
    val: i32,
}

impl A {
    fn mutate(&mut self) {
        self.read()
    }

    fn read(&self) {}
}

fn main() {
    let mut v = vec![1, 2, 3];

    for elem in v.iter_mut() {
        elem = 5; // Error
                  // *elem = 5;  // OK
    }
    println!("{:?}", v);
}
