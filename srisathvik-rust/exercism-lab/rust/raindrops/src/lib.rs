pub fn raindrops(number: u32) -> String {
    let mut result = String::new();

    if number % 3 == 0 {
        result.push_str("Pling");
    }

    if number % 5 == 0 {
        result.push_str("Plang");
    }

    if number % 7 == 0 {
        result.push_str("Plong");
    }

    if result.is_empty() {
        number.to_string()
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_of_3() {
        assert_eq!(raindrops(9), "Pling");
    }

    #[test]
    fn factors_of_3_and_5() {
        assert_eq!(raindrops(30), "PlingPlang");
    }

    #[test]
    fn factor_of_7() {
        assert_eq!(raindrops(28), "Plong");
    }

    #[test]
    fn factors_of_3_5_and_7() {
        assert_eq!(raindrops(105), "PlingPlangPlong");
    }

    #[test]
    fn no_matching_factors() {
        assert_eq!(raindrops(34), "34");
    }
}
