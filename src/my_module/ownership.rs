// practice ownership

pub fn practice_ownership() {
    // use String::from, we can combine String
    let mut s = String::from("hello");

    s.push_str(", rust!");

    println!("{}", s);
}

