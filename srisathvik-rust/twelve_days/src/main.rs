fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
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

    for day_index in 0..days.len() {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            days[day_index]
        );

        for gift_index in (0..=day_index).rev() {
            if gift_index == 0 && day_index > 0 {
                println!("and {}.", gifts[gift_index]);
            } else if gift_index == 0 {
                println!("{}.", gifts[gift_index]);
            } else {
                println!("{},", gifts[gift_index]);
            }
        }

        println!();
    }
}
