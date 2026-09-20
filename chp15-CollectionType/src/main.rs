// Common Collections

fn main() {
    // 1) Vectors
    // Read doc https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors
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

    // 2) UTF-8
    // Read doc https://doc.rust-lang.org/book/ch08-02-strings.html

    // 1
    let s = "whatever".to_string();
    // 2
    let s = String::from("whatever");
    // 3: Mutate the variable [push to it]
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("The value of s = {}", s);

    // Concetenating with + format
    let s1 = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3 = s1 + &s2; // note that s1 has be moved here and can no longer be use
    // refer explanation at https://doc.rust-lang.org/book/ch08-02-strings.html

    println!("cocatenated string is {}", s3);

    // Formatting Strings
    let salam = String::from("السلام عليكم");
    let salut = String::from("Hello");
    let full_message: String = format!("{salam} {salut}");
    println!("{full_message}");

    // 3) HashMap
    // Type is HashMap<K,V> stores a mapping of KV

    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();

    // populate the hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // print entire hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // get value for specific key
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("The score for team {team_name} is {score}");
}
