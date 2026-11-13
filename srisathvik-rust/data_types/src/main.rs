use std::io;
use std::num::ParseIntError;

fn parse_index(input: &str) -> Result<usize, ParseIntError> {
    input.trim().parse()
}

fn element_at(numbers: &[i32], index: usize) -> Option<i32> {
    numbers.get(index).copied()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index = parse_index(&input).expect("Index entered was not a number");

    match element_at(&numbers, index) {
        Some(element) => {
            println!("The value of the element at index {index} is: {element}");
        }
        None => {
            println!("Index {index} is outside the array.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_index() {
        // Arrange
        let input = " 2\n";

        // Act
        let result = parse_index(input);

        // Assert
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn rejects_non_numeric_index() {
        // Arrange
        let input = "two";

        // Act
        let result = parse_index(input);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn returns_element_for_valid_index() {
        // Arrange
        let numbers = [1, 2, 3, 4, 5];

        // Act
        let result = element_at(&numbers, 2);

        // Assert
        assert_eq!(result, Some(3));
    }

    #[test]
    fn returns_none_for_out_of_bounds_index() {
        // Arrange
        let numbers = [1, 2, 3, 4, 5];

        // Act
        let result = element_at(&numbers, 10);

        // Assert
        assert_eq!(result, None);
    }
}
