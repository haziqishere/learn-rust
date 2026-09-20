// Error Handling echniqus [ 2 approaches]

// Approach 1
// OPTION<T>
// Basically Enum that is used when a value that might be null

// Define the generic Opt type
// enum Option<T> {
//     Some(T), // Represents a value
//     None,    // Represents no value
// }

use std::num;

// Example:
fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Approach 2

// Define the generic Result type
// enum Result<T, E> {
//     Ok(T),   // Represents a value
//     ErrI(E), // Represents an error
// }

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot devide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    // Invoking first example
    let result = divideOption(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot devide by zero!"),
    }
    let result = divideOption(10.0, 5.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot devide by zero!"),
    }

    // Invoking second example
    match divideResult(10.23, 73.98) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match divideResult(10.23, 00.00) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
