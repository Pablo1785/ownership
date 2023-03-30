// 1. Move semantics
// Rust variable "assignment" in reality transfers the ownership of the value to the new variable
// The old variable no longer owns anything and therefore cannot be used

fn main() {
    let mut v1 = vec![1, 2, 3]; // Notice mutability
    let v2 = v1;
    v1 = vec![]; // Mutable variables can have a new value assigned
    println!("{:?}", v1); // Now it is fine to use the variable
}
