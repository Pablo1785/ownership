fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    let slice2 = &numbers[2..3];
    println!("{:?} {:?}", slice, slice2); // [2, 3, 4]
}
