struct A {
    val: i32,
}

impl A {
    fn mutate(&mut self) {
        self.read()
    }

    fn read(&self) {}
}

fn main() {}
