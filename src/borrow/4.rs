fn main() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v[0];
    let r2 = &v[1]; // Notice the error here (1 writer XOR N readers)
    v.push(4); // ...And here (1 writer)
    println!("{} {}", r1, r2);
}
