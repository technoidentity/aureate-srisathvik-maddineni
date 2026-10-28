fn main() {
    describe_number(Some(42));
    describe_number(None);
}

fn describe_number(optional_number: Option<i32>) {
    let Some(number) = optional_number else {
        println!("No number was provided.");
        return;
    };

    println!("The number is {number}.");
}
