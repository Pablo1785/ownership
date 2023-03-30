fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice); // [2, 3, 4]
}
