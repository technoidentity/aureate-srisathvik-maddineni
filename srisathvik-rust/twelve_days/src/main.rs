const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn verse(day: usize) -> Option<String> {
    if !(1..=12).contains(&day) {
        return None;
    }

    let day_index = day - 1;
    let mut result = format!(
        "On the {} day of Christmas, my true love gave to me:\n",
        DAYS[day_index]
    );

    for gift_index in (0..=day_index).rev() {
        if gift_index == 0 && day > 1 {
            result.push_str(&format!("and {}.", GIFTS[gift_index]));
        } else if gift_index == 0 {
            result.push_str(&format!("{}.", GIFTS[gift_index]));
        } else {
            result.push_str(&format!("{},\n", GIFTS[gift_index]));
        }
    }

    Some(result)
}

fn song() -> String {
    (1..=12)
        .filter_map(verse)
        .collect::<Vec<String>>()
        .join("\n\n")
}

fn main() {
    println!("{}", song());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_first_verse() {
        let result = verse(1).unwrap();

        let expected = concat!(
            "On the first day of Christmas, my true love gave to me:\n",
            "a partridge in a pear tree."
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn adds_and_after_the_first_day() {
        let result = verse(2).unwrap();

        assert!(result.contains("two turtle doves,"));
        assert!(result.contains("and a partridge in a pear tree."));
    }

    #[test]
    fn generates_middle_verse_in_reverse_gift_order() {
        let result = verse(5).unwrap();

        let five = result.find("five gold rings").unwrap();
        let four = result.find("four calling birds").unwrap();
        let three = result.find("three French hens").unwrap();

        assert!(five < four);
        assert!(four < three);
    }

    #[test]
    fn generates_twelfth_verse_with_all_gifts() {
        let result = verse(12).unwrap();

        assert!(result.starts_with("On the twelfth day"));
        assert!(result.contains("twelve drummers drumming"));
        assert!(result.ends_with("and a partridge in a pear tree."));
        assert_eq!(result.lines().count(), 13);
    }

    #[test]
    fn rejects_invalid_day_numbers() {
        assert_eq!(verse(0), None);
        assert_eq!(verse(13), None);
    }

    #[test]
    fn complete_song_contains_twelve_verses() {
        let result = song();

        assert_eq!(result.matches("day of Christmas").count(), 12);
        assert!(result.starts_with("On the first day"));
        assert!(result.contains("On the twelfth day"));
    }
}
