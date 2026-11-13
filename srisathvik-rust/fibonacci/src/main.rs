use std::io;

fn fibonacci(n: u32) -> Option<u64> {
    if n > 93 {
        return None;
    }

    if n == 0 {
        return Some(0);
    }

    if n == 1 {
        return Some(1);
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    Some(current)
}

fn main() {
    println!("Enter the Fibonacci position:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u32 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a valid non-negative whole number.");
            return;
        }
    };

    match fibonacci(n) {
        Some(result) => println!("Fibonacci number {n} is {result}"),
        None => println!("Please enter a position from 0 to 93."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_for_position_zero() {
        assert_eq!(fibonacci(0), Some(0));
    }

    #[test]
    fn returns_one_for_position_one() {
        assert_eq!(fibonacci(1), Some(1));
    }

    #[test]
    fn calculates_normal_fibonacci_position() {
        // Arrange
        let position = 10;

        // Act
        let result = fibonacci(position);

        // Assert
        assert_eq!(result, Some(55));
    }

    #[test]
    fn calculates_largest_supported_position() {
        assert_eq!(fibonacci(93), Some(12_200_160_415_121_876_738));
    }

    #[test]
    fn rejects_position_that_would_overflow() {
        assert_eq!(fibonacci(94), None);
    }
}
