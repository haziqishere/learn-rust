// In Rust, var as immutable by default

fn main() {
    println!("Hello, world!");
    let a = 5;
    println!("The value of a is {}", a);
    // a = 10; will "get cannot assign twice to immutable variable

    let mut b = 5;
    println!("The value of b is {}", b);
    b = 10;
    println!("The value of b is {}", b);
}
