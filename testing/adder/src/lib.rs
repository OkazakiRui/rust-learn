#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        if !(1..=100).contains(&value) {
            panic!(
                "予想値は1から100の間でなければなりませんが、{}でした.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::{add_two, Rectangle};

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("パニックを起こす");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let _larger = Rectangle {
            width: 8,
            height: 7,
        };
        let _smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // assert!(smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let _result = greeting("Carol");
        // assert!(
        //     result.contains("Carol"),
        //     "'Carol'は含まれませんでした, value was `{}`",
        //     result
        // );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic]
    fn greater_than_100_success() {
        // Guess::new(1);
        panic!("パニックを起こす");
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
