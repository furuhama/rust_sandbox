pub fn closure() {
    let n = 10;
    let plus_one = |x: i64| x + 1;
    let plus_one_twice = apply_twice(plus_one);

    println!("{}", plus_one_twice(n));
}

fn apply_twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
    move |x| f(f(x))
}
