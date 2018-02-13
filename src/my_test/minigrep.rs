// test for minigrep

#[cfg(test)]
mod test {
    use my_module;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            my_module::search(query, contents)
        );
    }
}
