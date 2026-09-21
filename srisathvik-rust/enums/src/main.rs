fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn add_fancy_hat() {
    println!("Added a fancy hat.");
}

fn remove_fancy_hat() {
    println!("Removed the fancy hat.");
}

fn reroll() {
    println!("Roll again.");
}
