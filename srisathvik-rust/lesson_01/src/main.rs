fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    let learner_name = "Srisathvik";

    println!("{}", greeting(learner_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_personalized_greeting() {
        // Arrange
        let learner_name = "Srisathvik";

        // Act
        let result = greeting(learner_name);

        // Assert
        assert_eq!(result, "Hello, Srisathvik!");
    }

    #[test]
    fn creates_greeting_for_another_name() {
        // Arrange
        let learner_name = "Rust learner";

        // Act
        let result = greeting(learner_name);

        // Assert
        assert_eq!(result, "Hello, Rust learner!");
    }
}
