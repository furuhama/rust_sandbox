// basic tests

#[cfg(test)]
mod tests {
    use my_module;
    use super::*;

    #[test]
    fn it_works() {
        my_module::hello_rust();
    }

    #[test]
    fn assertion_equal() {
        assert_eq!(2 + 2, 4);
    }

    // this test will fail
    //
    // #[test]
    // fn another() {
    //     panic!("Make this test fail!");
    // }

    #[test]
    fn larger_can_hold_smailler() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 2, width: 3 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 2, width: 3 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Nyanko");

        // basic assertion
        // assert!(result.contains("Nyanko"));

        assert!(
            result.contains("Nyanko"),
            // edit error message
            "Greeting did not contain name, value was '{}'", result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(28365);
    }

    // define struct & impl for test
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }

    // define function for test
    fn greeting(name: &str) -> String {
        format!("Hello, {}", name)
    }

    // define struct & impl for panic test
    #[derive(Debug)]
    struct Guess {
        value: u32,
    }

    impl Guess {
        fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, but got {}.", value);
            }

            Guess {
                value
            }
        }
    }
}
