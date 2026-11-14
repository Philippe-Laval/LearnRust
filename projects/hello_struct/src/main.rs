fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 : User = build_user("test@cool.com".to_string(), String::from("philippe"));

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("another@example.com"),
        ..user3
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black is {} {} {}", black.0, black.1, black.2);

    let Point(x, y, z) = origin;

    let subject = AlwaysEqual;


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;
