// set modules in my_tests

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
}
