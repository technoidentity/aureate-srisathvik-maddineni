fn main() {
    let message = String::from("hello");

    let (message, length) = calculate_length(message);

    println!("The length of '{message}' is {length}.");
}

fn calculate_length(text: String) -> (String, usize) {
    let length = text.len();

    (text, length)
}
