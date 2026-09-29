#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let large = Rectangle {
        width: 30,
        height: 50,
    };

    let small = Rectangle {
        width: 10,
        height: 40,
    };

    let square = Rectangle::square(20);

    println!("Large area: {}", large.area());
    println!("Can large hold small? {}", large.can_hold(&small));
    println!("Square: {square:#?}");
    println!("Square area: {}", square.area());
}
