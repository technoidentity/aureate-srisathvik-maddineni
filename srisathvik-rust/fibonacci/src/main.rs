use std::io;

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

    if n > 93 {
        println!("Please enter a position from 0 to 93.");
        return;
    }

    let result = fibonacci(n);

    println!("Fibonacci number {n} is {result}");
}

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut previous = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    current
}
