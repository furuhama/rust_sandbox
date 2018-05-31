// test Pattern matching

pub fn pattern_matching() {
    let opt: Option<i32> = None;
    let unwrapped = match opt {
        Some(v) => v,
        None => 100,
    };

    println!("the result is: {}", unwrapped);
}
