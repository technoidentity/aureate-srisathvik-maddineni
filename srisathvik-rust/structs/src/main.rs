struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let first_user = build_user(
        String::from("srisathvik"),
        String::from("first@example.com"),
    );

    let second_user = User {
        email: String::from("second@example.com"),
        ..first_user
    };

    println!("Second username: {}", second_user.username);
    println!("Second email: {}", second_user.email);
    println!("Second user active: {}", second_user.active);
    println!("Second user sign-in count: {}", second_user.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;

    println!("Black RGB: {}, {}, {}", black.0, black.1, black.2);
    println!(
        "Origin coordinates: {}, {}, {}",
        origin.0, origin.1, origin.2
    );
    println!("Created a unit-like AlwaysEqual value.");
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
