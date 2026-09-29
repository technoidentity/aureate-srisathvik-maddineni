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
