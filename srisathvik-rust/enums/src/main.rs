fn describe_number(optional_number: Option<i32>) -> String {
    let Some(number) = optional_number else {
        return String::from("No number was provided.");
    };

    format!("The number is {number}.")
}

fn main() {
    println!("{}", describe_number(Some(42)));
    println!("{}", describe_number(None));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_present_number() {
        // Given
        let optional_number = Some(42);

        // When
        let result = describe_number(optional_number);

        // Then
        assert_eq!(result, "The number is 42.");
    }

    #[test]
    fn describes_missing_number() {
        // Given
        let optional_number = None;

        // When
        let result = describe_number(optional_number);

        // Then
        assert_eq!(result, "No number was provided.");
    }
}
