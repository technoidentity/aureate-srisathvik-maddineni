use std::io;

use raindrops::raindrops;

fn main() {
    println!("Enter a positive whole number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u32 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a valid positive whole number.");
            return;
        }
    };

    let result = raindrops(number);

    println!("Raindrop result: {result}");
}
