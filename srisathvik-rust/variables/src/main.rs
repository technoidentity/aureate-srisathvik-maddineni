const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutable variable
    let mut number = 5;
    println!("The initial number is: {number}");

    number = 6;
    println!("The changed number is: {number}");

    // Shadowing and scope
    let value = 5;
    let value = value + 1;

    {
        let value = value * 2;
        println!("The value in the inner scope is: {value}");
    }

    println!("The value in the outer scope is: {value}");

    // Shadowing can change the type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    // Constant
    println!("Three hours contains {THREE_HOURS_IN_SECONDS} seconds.");
}
