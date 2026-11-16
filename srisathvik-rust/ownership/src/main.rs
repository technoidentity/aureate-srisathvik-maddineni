fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..index];
        }
    }

    text
}

fn middle_three(numbers: &[i32]) -> Option<&[i32]> {
    numbers.get(1..4)
}

fn main() {
    let sentence = "hello world";
    let word = first_word(sentence);

    println!("The first word is: {word}");

    let numbers = [10, 20, 30, 40, 50];
    let middle = middle_three(&numbers).expect("Array must contain at least four elements");

    println!("Array slice: {middle:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_first_word_from_sentence() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn returns_entire_text_when_there_is_no_space() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn returns_empty_slice_for_empty_text() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn returns_empty_slice_when_text_starts_with_space() {
        assert_eq!(first_word(" hello"), "");
    }

    #[test]
    fn returns_middle_three_elements() {
        // Arrange
        let numbers = [10, 20, 30, 40, 50];

        // Act
        let result = middle_three(&numbers);

        // Assert
        assert_eq!(result, Some(&[20, 30, 40][..]));
    }

    #[test]
    fn returns_none_when_slice_is_too_short() {
        let numbers = [10, 20, 30];

        assert_eq!(middle_three(&numbers), None);
    }
}
