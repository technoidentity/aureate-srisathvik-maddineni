fn main() {
    let x = five();
    let y = plus_one(x);

    println!("five() returned: {x}");
    println!("plus_one({x}) returned: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
