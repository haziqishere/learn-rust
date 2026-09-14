// Ownership, Borrowing and References

// Ownership
// ---------
// There are lang that lets u control mem such C & C++. Allow u to reserve part of mem and release it once finish
// Problem it cause: You might forgot to release memory [Memory Management Control Issue]
// How the lang solved this? : Garbage Collect
// What does the new issue introduced? : This ops run at runtime. GC require it to temporary pause to program to clean mem
//                                       Only then it resumes the program. Not ideal in-terms of time

// What is Ownership and how it resolves above issue?
// Every value has a single owner [every variable has one value, and its sole owner]
// Rules in ownership:
// 1. Each value in Rust has an owner
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Example:
fn main() {
    // Rule 1: Each value in Rust has a variable that's its owner
    let s1 = String::from("RUST"); // The RUST value is owned by s1
    let len = calculate_length(&s1); // We'll not pass the owner, but we'll pass the ref to that owner
    println!("Length of '{}' is {}.", s1, len);

    // Rule 2: There can only be one owner at a time
    let s2 = s1; // We transfer that ownership from s1 -> s2
                 // println!("{}", s1)   // Will cause error `error[E0382]: borrow of moved value: `s1`
    println!("{}", s2); // Compile normally

    // Rule 3: When the owner goes out of scope, the value will be dropped.

    // To *see* rule 3 happen, wrap value in struct implementing Drop.
    // Rust calls .drop() automatically when owner leaves scope - no manual free.
    {
        let d1 = Droppable {
            name: String::from("d1"),
        };
        println!("{} created", d1.name);
    } // <- d1 goes out of scope HERE, drop() fires automatically

    println!("back in main, d1 is gone");

    let d2 = Droppable {
        name: String::from("d2"),
    };
    println!("{} created, will drop at end of main", d2.name);
}

struct Droppable {
    name: String,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
