fn choose_number(condition: bool) -> i32 {
    if condition { 5 } else { 6 }
}

fn main() {
    let condition = true;
    let number = choose_number(condition);

    println!("The value of number is: {number}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_five_when_condition_is_true() {
        // Arrange
        let condition = true;

        // Act
        let result = choose_number(condition);

        // Assert
        assert_eq!(result, 5);
    }

    #[test]
    fn chooses_six_when_condition_is_false() {
        // Arrange
        let condition = false;

        // Act
        let result = choose_number(condition);

        // Assert
        assert_eq!(result, 6);
    }
}
