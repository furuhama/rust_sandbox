// practice ownership

pub fn practice_ownership() {
    // use String::from, we can combine String
    let mut s = String::from("hello");

    s.push_str(", rust!");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}
