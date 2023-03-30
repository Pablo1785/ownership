fn main() {
    let mut v = vec![1, 2, 3];
    borrow_fn(&v[0], &v[1]);
    v.push(4); // Mutable borrowing here is fine because the previous references will be dropped when they reach the end of the function
}

fn borrow_fn(r1: &i32, r2: &i32) {
    println!("{} {}", *r1, *r2);
}
