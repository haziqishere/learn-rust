// Shadowing
// textbook def: Fist variable is shadowed by the second, which means that the second variable is what
//               the compiler will see when u use the name of the variable.

// U can declare the same variable name as prev variable.
fn main() {
    let x = 5;

    let x = x + 1; // This is shadowing. The first x was shadowed by 2nd x

    {
        let x = x * 2;
        println!("The final value of x is {}", x); // 3rd x shadow 2nd x
    }

    // NOTE: Shadowing is not the same as marking a var as mutable
    // x = 10;  // raises "cannot assign twice to immutable variable"

    // and u must explicitly use 'let'
    x = x * 2; // raises "cannot assign twice to immutable variable"
}
