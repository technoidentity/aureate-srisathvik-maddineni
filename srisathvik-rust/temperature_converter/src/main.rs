use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Choose 1 or 2:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice");

    println!("Enter the temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    match choice.trim() {
        "1" => {
            let result = celsius_to_fahrenheit(temperature);
            println!("{temperature}°C = {result}°F");
        }
        "2" => {
            let result = fahrenheit_to_celsius(temperature);
            println!("{temperature}°F = {result}°C");
        }
        _ => println!("Invalid choice. Please choose 1 or 2."),
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_approximately_equal(actual: f64, expected: f64) {
        let difference = (actual - expected).abs();

        assert!(
            difference < 0.000_001,
            "expected {expected}, but received {actual}"
        );
    }

    #[test]
    fn converts_celsius_freezing_point_to_fahrenheit() {
        let result = celsius_to_fahrenheit(0.0);

        assert_approximately_equal(result, 32.0);
    }

    #[test]
    fn converts_celsius_boiling_point_to_fahrenheit() {
        let result = celsius_to_fahrenheit(100.0);

        assert_approximately_equal(result, 212.0);
    }

    #[test]
    fn converts_fahrenheit_freezing_point_to_celsius() {
        let result = fahrenheit_to_celsius(32.0);

        assert_approximately_equal(result, 0.0);
    }

    #[test]
    fn converts_fahrenheit_boiling_point_to_celsius() {
        let result = fahrenheit_to_celsius(212.0);

        assert_approximately_equal(result, 100.0);
    }

    #[test]
    fn converts_negative_celsius_temperature() {
        let result = celsius_to_fahrenheit(-40.0);

        assert_approximately_equal(result, -40.0);
    }

    #[test]
    fn converts_negative_fahrenheit_temperature() {
        let result = fahrenheit_to_celsius(-40.0);

        assert_approximately_equal(result, -40.0);
    }
}
