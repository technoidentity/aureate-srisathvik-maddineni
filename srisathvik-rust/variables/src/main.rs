const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn change_number(mut number: i32, replacement: i32) -> (i32, i32) {
    let initial_number = number;
    number = replacement;

    (initial_number, number)
}

fn shadowed_values(starting_value: i32) -> (i32, i32) {
    let value = starting_value;
    let value = value + 1;

    let outer_value = value;

    let inner_value = {
        let value = value * 2;
        value
    };

    (outer_value, inner_value)
}

fn text_length(text: &str) -> usize {
    let spaces = text;
    let spaces = spaces.len();

    spaces
}

fn main() {
    let (initial_number, changed_number) = change_number(5, 6);

    println!("The initial number is: {initial_number}");
    println!("The changed number is: {changed_number}");

    let (outer_value, inner_value) = shadowed_values(5);

    println!("The value in the inner scope is: {inner_value}");
    println!("The value in the outer scope is: {outer_value}");

    let spaces = text_length("   ");
    println!("Number of spaces: {spaces}");

    println!("Three hours contains {THREE_HOURS_IN_SECONDS} seconds.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changes_mutable_number() {
        let result = change_number(5, 6);

        assert_eq!(result, (5, 6));
    }

    #[test]
    fn calculates_outer_shadowed_value() {
        let (outer_value, _) = shadowed_values(5);

        assert_eq!(outer_value, 6);
    }

    #[test]
    fn calculates_inner_shadowed_value() {
        let (_, inner_value) = shadowed_values(5);

        assert_eq!(inner_value, 12);
    }

    #[test]
    fn counts_characters_after_type_shadowing() {
        assert_eq!(text_length("   "), 3);
        assert_eq!(text_length("Rust"), 4);
        assert_eq!(text_length(""), 0);
    }

    #[test]
    fn constant_contains_three_hours_in_seconds() {
        assert_eq!(THREE_HOURS_IN_SECONDS, 10_800);
    }
}
