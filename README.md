# Rust Survival Guide - Ownership, borrowing, references, slices, lifetimes

This is the most offputting part of the Rust programming language to newcomers. The borrow checker is the thing that differentiates Rust from other programming languages, giving unique benefits like no dangling references and no null pointer dereference errors at runtime. Unfortunately this means that certain algorithms, expecially self-referential data structures can be hard to implement properly. Implementing self-referential, recursive data structures will feel like an uphill battle at first.

Fortunately there are ways to avoid the hassle with borrow checking so you can still reap all the rewards of a strong and expressive type system that Rust offers, while maintaining its high performance.

This repo aims to be a survival guide that will explain and demonstrate the principles of borrow checking and give tips on how to avoid headaches when building your first Rust application.

# Ownership

Rust has a unique ```ownership``` model used to provide memory safety and prevent common mistakes, such as null pointer dereferencing and use-after-free errors.

Each value in memory has an owner which is the variable that allocated the memory. The ```owner``` has the exclusive right to modify or destroy the value. When the owner goes out of scope, the value is cleaned up (```dropped```) and the memory is freed.

This ownership is enforced at compile time rather than on runtime, which means any code that is run adheres to the ownership rules. Compiler analyzes the code to see if every value has only a single owner and the ownership is transferred correctly. This analysis is called the ```borrow checker```.

Assigning a variable to a new variable actually passes the ownership of the value to the new variable. The old one can no longer be borrowed or assigned to other variables.

Ownership rules:

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time for a value.
3. When the owner goes out of scope, the value will be dropped.

```rust
// Rust variable "assignment" in reality transfers the ownership of the value to the new variable
// The old variable no longer owns anything and therefore cannot be used

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    println!("{:?}", v1); // Notice the error here (move); Also notice the compiler's help message
}

```

# Borrowing

To facilitate unique ownership without copying large amounts of data Rust uses ```borrowing``` and ```lifetimes```. Borrowing allows a function to temporarily reference a value owned by another function, without taking ownership of it.

These borrowed values work similar to references in other programming languages.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r1 = &v[0];
    let r2 = &v[1];
    v.push(4); // Notice the error here (1 writer XOR N readers)
    println!("{} {}", r1, r2); // Removing this line also removes the error; the compiler is smart enough to see
                               //       that the reference variables are never actually read
}
```

# Slices

Slices allow you to borrow a contiguous fragment of data from a collection.

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];
    println!("{:?}", slice); // [2, 3, 4]
}
```

# *String types

There are two main string types in Rust:

1. ```String``` is a heap-allocated string type that allows you to modify and grow its contents. It's implemented as a vector of bytes (```Vec<u8>```) with some additional convenience methods for working with strings.

2. ```&str``` is a ```string slice``` - a reference to a stack-allocated

# *Arrays vs Vectors vs Slices

### Arrays

Arrays are a fixed-size collection of elements of the same type, and they are allocated on the stack. Once an array is created, its size cannot be changed. Array elements can be accessed using their index.

```rust
fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array); // [1, 2, 3, 4, 5]
}
```

### Slices

Slices, as we saw in the previous section, are references to a contiguous sequence of elements in a collection. Slices are denoted by a range of indices and are used to reference a portion of an array or vector. Slices are created using the & operator and the range notation.

```rust
fn main() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4];
    println!("{:?}", slice); // [2, 3, 4]
}
```

### Vectors

Vectors, or Vec for short, are a growable collection of elements of the same type, and they are allocated on the heap. Vectors can be created with an initial capacity, and can grow or shrink as needed. Vectors can be accessed using their index, just like arrays.

```rust
fn main() {
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.push(6);
    println!("{:?}", vector); // [1, 2, 3, 4, 5, 6]
}
```

# Lifetimes

Lifetimes ensure that borrowed values (references) are not used after the original data was dropped. This prevents dangling references and use-after-free errors.

Every borrowed value has a ```lifetime``` which means they will become invalid after the original value is dropped.

Generic Lifetime Annotations (GLA) are also commonly referred to as ```lifetimes``` and you will often see people refer to GLAs like that. These are annotations on borrowed values that specify how long different references will be valid for, as well as the relationships between those durations, e.x. reference A outlives reference B.

No lifetimes required here:
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    borrow_fn(&v[0], &v[1]);
    v.push(4); // Mutable borrowing here is fine because the previous references will be dropped when they reach the end of the function
}

fn borrow_fn(r1: &i32, r2: &i32) {
    println!("{} {}", *r1, *r2);
}
```

Imagine you are the compiler trying to figure out if the returned reference is valid. You cannot possibly know the outcome of the if-else at compile time.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r = borrow_fn(&v[0], &v[1]);
    println!("{}", r);
}

fn borrow_fn(r1: &i32, r2: &i32) -> &i32 {
    println!("{} {}", *r1, *r2);
    if r1 > r2 {
        r1
    } else {
        r2
    }
}
```

Generic lifetime annotations (lifetimes) give the compiler necessary information to determine if returned references will be valid.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let r = borrow_fn(&v[0], &v[1]);
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
```

# Tips

1. Avoid borrowing values in structs, enums, and collections.
    - Use String instead of &str, Vec<T> instead of &[T]
    - Similarly, use Vec<T> instead of Vec<&T>, HashMap<String, T> instead of HashMap<&str, T>
2. Use #[derive(Clone)] and .clone() to physically copy values. Copy is quite convenient because it changes the behavior of the assignment operator from move semantics to copy semantics, so you don't have to write .clone(), but it can cause unintended copies.

Here's a quick snippet about borrowing two mutable slices from the same collection at the same time. There are two ways to work around this:

a) Vec has the method .split_mut() and .split_at_mut() which give us two slices without copying data.

b) Use an iterator. The Iterator trait contains a huge number of helpful methods for this problem.

c) Copy slices to Vectors.

3. Use smart pointers to build recursive data structures. It is possible to do this without them, but it is a deep rabbit hole and not recommended because you can quickly become discouraged. Link to the linked list tutorial is at the end of the presentation

4. *Use smart pointers to circumvent ownership and borrow checker rules. RefCell, Rc, Arc, and other smart pointers allow us to shift borrow checking from compile time to runtime. Here, of course, we must be careful and use the minimum working pointer. Pointer cheat sheet is at the end of the presentation

5. As a side note, use .unwrap() and .expect() to unpack Option<> and Result<> types.