// struct
struct User {
    active: bool, // field
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("2wPjA@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);

    // mutable
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user2.email);
    user2.email = String::from("vLbZx@example.com"); // mutable
    println!("{}", user2.email);

    // function
    let user3 = build_user(String::from("user3@example.com"), String::from("someusername123"));
    println!("{}", user3.email);
}

// function
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}