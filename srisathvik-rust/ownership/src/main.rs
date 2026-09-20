fn main() {
    let message = create_message();

    println!("{message}");
}

fn create_message() -> String {
    let message = String::from("hello");

    message
}
