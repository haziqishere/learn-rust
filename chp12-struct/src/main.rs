// Structs
// Like tuples but more flexible

fn main() {
    // tuple
    let rect = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1: User = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@mail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@mail.com");
    print!("User email is {}", user1.email);

    // Return a struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // Create instances from other instances
    let user2: User = User {
        email: String::from("another@mail.com"),
        ..user1
    };

    // Tuples Struct
    // No name field
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let white: Color = Color(255, 255, 255);

    // Unit-Like struct
    // no field and used when need a type to store trait but no need to store data
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // or more familiar
    struct Completed;
    let pipeline_status = Completed;
}
