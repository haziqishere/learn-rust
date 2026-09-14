// IF Else [ If expression ] [ Else expression ]
// branch your expression according to condition

#![allow(warnings)]
fn main() {
    let age: u16 = 18;

    if age >= 18 {
        println!("You can drive a car!")
    } else {
        println!("You can't drive a car")
    }

    // Multiple conditions with elif
    let num = 6;

    if num % 4 == 0 {
        println!("Number is divisible by 4")
    } else if num % 3 == 0 {
        println!("Number is divisible by 3")
    } else if num % 2 == 0 {
        println!("Number is divisible by 2")
    }

    // Inline if else
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("Number: {number}")
}
