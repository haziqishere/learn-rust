// Repetition with loops
// Doing things over and over
// 3 types of loops keyword:
//  1. Loop
//  2. While
//  3. For

fn main() {
    // 1. Loop keyword
    // cont running until explicitly tell it to stop
    // loop {
    //     println!("Hell, World!")
    // } // run hello world infinitely

    // Return values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // use break to stop loop
        }
    };

    println!("The result is {result}");
    println!(" ");
    // Loop Labels to Disambiguate between multple loops

    // example with nested loop
    let mut count = 0;
    'counting_up: loop {
        // 'counting_up is a label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // only break inner loop
            }
            if count == 2 {
                break 'counting_up; // break outer loop
            }

            remaining -= 1;
            count += 1;
        }
    }

    println!("");
    // 2. While loop
    // conditional loop. When condition ceases to be true
    // the program calls break stoping the loop

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Hey!");

    println!("");
    // 3. For loop
    // use for looping thru element

    let a = [1, 2, 3, 4, 5, 6];

    for element in a {
        println!("{element}");
    }
}
