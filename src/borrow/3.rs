fn main() {
    let mut v = vec![1, 2, 3];
    let r = borrow_fn(&v[0], &v[1]);
    v.push(4); // Notice the error here (1 writer XOR N readers)
    println!("{}", r);
}

fn borrow_fn<'a>(r1: &'a i32, r2: &'a i32) -> &'a i32 {
    println!("{} {}", *r1, *r2);
    if r1 > r2 {
        r1
    } else {
        r2
    }
}
