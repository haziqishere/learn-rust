// Primitives datatypes (also known as scalar datatypes)
// int, float, bool, char

//Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes
// i8, i16, i32, i64, i128 : Signed integers
// u8, u16, u32, u64, u128 : Unsigned integers

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;   // only +ve val

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
// diff between i32 (32 bits) and i64(64 bits)
// range i32: - -2,147,483,648 to 2,147,483,647
// range i64: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    let e: i32 = 2147483647; // max value of i32
    let i: i64 = 9223372036854775807; // max value of i64

    println!("Max value of i32: {}", e);
    println!("Max value of i64: {}", i);

// ---------------------------------------------------------------------
//Floats [Floating Point Types]
// f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi {}",pi);

// ---------------------------------------------------------------------
// Boolean Values: true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

// ---------------------------------------------------------------------
// Character Type - char
    let letter: char = 'a';
    println!("First letter of alphabet is {}", letter);
}
