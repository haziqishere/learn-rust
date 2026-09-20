// Common Collections

// 1) Vectors
// Read doc https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors

fn main() {
    let mut _v: Vec<i32> = Vec::new(); // new is a method that says to rust "I want to create new vector"
    let mut _v: Vec<i32> = vec![1, 2, 3];

    // append to the vec
    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);
    _v.push(9);

    println!("{:?}", _v);

    // access specific index in vector:
    let third: &i32 = &_v[2]; // method a) Direct indexing
    println!("The 3rd element is {}", third);

    let third: Option<&i32> = _v.get(2); // method b) Use vector method
    match third {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no thrid element."),
    }
}
