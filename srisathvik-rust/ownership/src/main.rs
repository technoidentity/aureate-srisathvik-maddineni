fn main() {
    let sentence = "hello world";
    let word = first_word(sentence);

    println!("The first word is: {word}");

    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4];

    println!("Array slice: {middle:?}");
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..index];
        }
    }

    text
}
