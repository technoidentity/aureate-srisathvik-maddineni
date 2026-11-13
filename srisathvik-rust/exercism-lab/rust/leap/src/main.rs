use std::io;

use leap::is_leap_year;

fn main() {
    println!("Enter a year:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let year: u64 = match input.trim().parse() {
        Ok(year) => year,
        Err(_) => {
            println!("Please enter a valid positive whole year.");
            return;
        }
    };

    if is_leap_year(year) {
        println!("{year} is a leap year.");
    } else {
        println!("{year} is not a leap year.");
    }
}
