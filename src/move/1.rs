// 1. Move semantics
// Rust variable "assignment" in reality transfers the ownership of the value to the new variable
// The old variable no longer owns anything and therefore cannot be used

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    println!("{:?}", v1); // Notice the error here (move); Also notice the compiler's help message
}
