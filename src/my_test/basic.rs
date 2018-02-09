// basic tests

#[cfg(test)]
mod tests {
    use my_module;

    #[test]
    fn it_works() {
        my_module::hello_rust();
    }

    #[test]
    fn assertion_equal() {
        assert_eq!(2 + 2, 4);
    }

    // this test will fail
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

    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }
}
