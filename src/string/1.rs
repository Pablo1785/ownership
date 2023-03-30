fn main() {
    let s = "mój string";

    // Create owned data from borrowed data
    let owned_s = s.to_string(); // String specific to_owned()
    let owned_s2 = s.to_owned(); // Generic to_owned()

    // Borrow data
    let ref_s = owned_s.as_str(); // String specific way of extracting a slice
    let ref_s2 = &owned_s2[..]; // Generic slice syntax

    println!("{}", owned_s); // We have to use interpolation because println! macro only accepts literals
    println!("{}", owned_s2);

    println!("{}", ref_s); // Same goes for str slices
    println!("{}", ref_s2);
}
