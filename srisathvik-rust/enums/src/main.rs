fn main() {
    let number = 5;

    let present_number = Some(10);
    let present_value = present_number.unwrap_or(0);

    let absent_number: Option<i32> = None;
    let absent_value = absent_number.unwrap_or(0);

    println!("Present result: {}", number + present_value);
    println!("Absent result: {}", number + absent_value);
}
