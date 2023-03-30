// You can get temporary access to a value by borrowing it; this creates a reference
// You can have EITHER 1 mutable (read and write) reference to a variable OR any number
//      of immutable (read-only) references; never both

fn main() {
    let mut v = vec![1, 2, 3];
    let r1 = &v[0];
    let r2 = &v[1];
    v.push(4); // Notice the error here (1 writer XOR N readers)
    println!("{} {}", r1, r2); // Removing this line also removes the error; the compiler is smart enough to see
                               //       that the reference variables are never actually read
}
