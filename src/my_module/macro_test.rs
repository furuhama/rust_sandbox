pub fn try_macro() {
    println!("{}", anything_to_string());
}

fn anything_to_string<'a>() -> &'a str {
    stringify!(hogehoge)
}
