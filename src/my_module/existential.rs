pub fn existential() {
    let f = fn_trait();

    println!("{}", f(41));
}

fn fn_trait() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
