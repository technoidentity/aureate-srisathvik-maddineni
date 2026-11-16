use std::cmp::Ordering;
use std::io;

use rand::Rng;

#[derive(Debug, PartialEq, Eq)]
enum GuessResult {
    TooSmall,
    TooBig,
    Correct,
}

fn parse_guess(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

fn check_guess(guess: u32, secret_number: u32) -> GuessResult {
    match guess.cmp(&secret_number) {
        Ordering::Less => GuessResult::TooSmall,
        Ordering::Greater => GuessResult::TooBig,
        Ordering::Equal => GuessResult::Correct,
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let Some(guess) = parse_guess(&input) else {
            println!("Please enter a valid whole number.");
            continue;
        };

        println!("You guessed: {guess}");

        match check_guess(guess, secret_number) {
            GuessResult::TooSmall => println!("Too small!"),
            GuessResult::TooBig => println!("Too big!"),
            GuessResult::Correct => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_guess() {
        assert_eq!(parse_guess(" 42\n"), Some(42));
    }

    #[test]
    fn rejects_invalid_guess() {
        assert_eq!(parse_guess("forty-two"), None);
    }

    #[test]
    fn identifies_guess_as_too_small() {
        // Given
        let secret_number = 50;

        // When
        let result = check_guess(25, secret_number);

        // Then
        assert_eq!(result, GuessResult::TooSmall);
    }

    #[test]
    fn identifies_guess_as_too_big() {
        let result = check_guess(75, 50);

        assert_eq!(result, GuessResult::TooBig);
    }

    #[test]
    fn identifies_correct_guess() {
        let result = check_guess(50, 50);

        assert_eq!(result, GuessResult::Correct);
    }
}
