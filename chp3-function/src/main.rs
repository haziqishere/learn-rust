// Functions
// Tips:
// 1) Best practice is to write function & variables in snake_case
// 2) Rust function support Hoisting - can call function anywhere in code

// Entry point. Rust expects everything to run under main func
fn main() {
    hello_world();
    tell_height(182); // pass single arg
    human_id("Haziq", 24, 169.3); // pass multiple args
    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is: {}", _x);

    // Assigning func output to a variable
    let y: i32 = add(4, 6);
    println!("Value of y is: {}", y);
    // invoking func on-the-fly in other executables
    println!("Value from func add is {}", add(4, 6));

    // Calling BMI func
    let weight: f64 = 70.0;
    let height: f64 = 1.695;
    let bmi: f64 = get_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}
fn hello_world() {
    println!("Hello, Rust!");
}

// can declare the function params
fn tell_height(height: u32) {
    println!("My heigt is {} cm.", height);
}

// can declare more function params
fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {}cm.",
        name, age, height
    );
}

// Expressions and Statements
// What is Expression? : Anything that returns a value
// What is Statement? : Anything that does not return a value. Almost all end with ;

// function returning values
fn add(a: i32, b: i32) -> i32 {
    a + b // Example of expression
}

// Final Example
// BMI = weight(kg) / height(m)^2

fn get_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
