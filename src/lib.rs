// set original modules in this file

pub mod my_module;

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
}
