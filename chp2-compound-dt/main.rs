// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

fn main() {
    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);

    // NOTE: Below will cause error the array contain mix datatype
    // let mix = [1,2,"apple",true]
    // println!("Print: {:?}", mix)

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);
    // ------------------------------------------------------------------------
    // Tuples
    // like-Array but support for mix value
    let human1: (String, i32, bool) = ("Alice".to_string(), 30, false); // "Alice" is slice string type, not pure string
    let human2 = ("Chuck", 55, true, [1, 2, 3, 4, 5]);

    println!("Human Tuples: {:?}", human1);
    println!("Human Tuples: {:?}", human2);

    // ------------------------------------------------------------------------
    // Slices: Dynamically sized view of contiguous sequence of element. What it means is
    //         set of elements placed together in a continous, unbroken order /wo any gaps.
    //         On phycial perspective, tis series of data elements stored right next to each other in memory
    //         Example: if list is [1,2,3,4], then [2,3] is a contigous sequence

    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"]; // Disclaimer: &str is string slice
    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];

    println!("Number Slices: {:?}", number_slices);
    println!("Animal Slices: {:?}", animal_slices);
    println!("Book Slices: {:?}", book_slices);
    // ------------------------------------------------------------------------

    // Strings VS String Slices (&str)

    // Strings [growable, mutable, owned string type]
    // stores in heap memory
    let mut stone_cold: String = String::from("Hell"); // By default, all rust var are immutable until u put `mut`
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice)
    // Its a reference to a string literals /substring somewhere in ur code/memory without needing to copy or own that data.
    // Benefit for memory efficiency. Just like how Polar's dataframe works.
    let string_sample: String = String::from("Hello, World"); // declare a string
    let slice: &str = &string_sample;
    let _slice_hello: &str = &string_sample[0..5];

    println!("Slice Value: {}", slice);
    println!("Slice Hello: {}", _slice_hello);
}

// NOTE: Below will cause error. Rust cleans memory so whatver var declared above cannot be simply invoked in other func
// fn print() {
//     println!("SLICE: {}", slice)
// }
