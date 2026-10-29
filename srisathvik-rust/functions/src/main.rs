fn main() {
    let x = five();
    let y = plus_one(x);

    println!("five() returned: {x}");
    println!("plus_one({x}) returned: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_returns_five() {
        // Act
        let result = five();

        // Assert
        assert_eq!(result, 5);
    }

    #[test]
    fn plus_one_increments_positive_number() {
        // Arrange
        let number = 5;

        // Act
        let result = plus_one(number);

        // Assert
        assert_eq!(result, 6);
    }

    #[test]
    fn plus_one_increments_negative_number() {
        // Arrange
        let number = -1;

        // Act
        let result = plus_one(number);

        // Assert
        assert_eq!(result, 0);
    }
}
