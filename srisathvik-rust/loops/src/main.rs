fn value_messages(values: &[i32]) -> Vec<String> {
    let mut messages = Vec::new();

    for element in values {
        messages.push(format!("The value is: {element}"));
    }

    messages
}

fn countdown(start: u32) -> Vec<u32> {
    let mut numbers = Vec::new();

    for number in (1..=start).rev() {
        numbers.push(number);
    }

    numbers
}

fn main() {
    let values = [10, 20, 30, 40, 50];

    for message in value_messages(&values) {
        println!("{message}");
    }

    for number in countdown(3) {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_message_for_each_value() {
        // Arrange
        let values = [10, 20, 30];

        // Act
        let result = value_messages(&values);

        // Assert
        assert_eq!(
            result,
            vec!["The value is: 10", "The value is: 20", "The value is: 30"]
        );
    }

    #[test]
    fn creates_reverse_countdown() {
        assert_eq!(countdown(3), vec![3, 2, 1]);
    }

    #[test]
    fn creates_single_number_countdown() {
        assert_eq!(countdown(1), vec![1]);
    }

    #[test]
    fn creates_empty_countdown_from_zero() {
        assert!(countdown(0).is_empty());
    }
}
