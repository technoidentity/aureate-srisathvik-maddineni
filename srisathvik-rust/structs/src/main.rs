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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_rectangle_area() {
        let rectangle = Rectangle {
            width: 30,
            height: 50,
        };

        assert_eq!(rectangle.area(), 1500);
    }

    #[test]
    fn larger_rectangle_can_hold_smaller_rectangle() {
        let large = Rectangle {
            width: 30,
            height: 50,
        };
        let small = Rectangle {
            width: 10,
            height: 40,
        };

        assert!(large.can_hold(&small));
    }

    #[test]
    fn rectangle_cannot_hold_a_wider_rectangle() {
        let rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        let wider = Rectangle {
            width: 40,
            height: 20,
        };

        assert!(!rectangle.can_hold(&wider));
    }

    #[test]
    fn equal_rectangle_cannot_be_held() {
        let rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        let equal = Rectangle {
            width: 30,
            height: 50,
        };

        assert!(!rectangle.can_hold(&equal));
    }

    #[test]
    fn creates_square_with_equal_dimensions() {
        let square = Rectangle::square(20);

        assert_eq!(square.width, 20);
        assert_eq!(square.height, 20);
        assert_eq!(square.area(), 400);
    }
}
