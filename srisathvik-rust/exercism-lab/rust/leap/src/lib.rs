pub fn is_leap_year(year: u64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_divisible_by_4_is_a_leap_year() {
        assert!(is_leap_year(1996));
    }

    #[test]
    fn year_divisible_by_100_is_not_a_leap_year() {
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn year_divisible_by_400_is_a_leap_year() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn normal_year_is_not_a_leap_year() {
        assert!(!is_leap_year(2023));
    }
}
