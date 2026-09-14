// Constants
// values bound to name but not allow to change. Hence, not allow to use mut

fn main() {
    // const mut y = 10;   // surfaced error "const globals cannot be mutable"
    const Y: i32 = 10; // have to be capital letter

    println!("constant Y is {}", Y);
    println!("value of pi is {}", PI);
    println!("value of 3 hours in seconds {}", THREE_HOURS_IN_SECONDS);
}

// You can declare a constant here with a type annotations.
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
