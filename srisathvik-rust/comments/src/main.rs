fn lucky_number_message(number: i32) -> String {
    format!("My lucky number is: {number}")
}

fn main() {
    // This comment is on its own line.
    let lucky_number = 7; // This is an end-of-line comment.

    // Each line of a multi-line explanation can begin with `//`.
    // Rust ignores these lines when compiling the program.

    /*
       Rust also supports block comments.
       Everything between these markers is ignored.
    */

    println!("{}", lucky_number_message(lucky_number));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_message_for_lucky_number_seven() {
        // Arrange
        let lucky_number = 7;

        // Act
        let result = lucky_number_message(lucky_number);

        // Assert
        assert_eq!(result, "My lucky number is: 7");
    }

    #[test]
    fn creates_message_for_another_number() {
        // Arrange
        let lucky_number = 21;

        // Act
        let result = lucky_number_message(lucky_number);

        // Assert
        assert_eq!(result, "My lucky number is: 21");
    }
}
