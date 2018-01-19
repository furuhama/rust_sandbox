// practice ownership

pub fn practice_ownership() {
    // use String::from, we can combine String
    let mut s = String::from("hello");

    s.push_str(", rust!");

    // String move its ownership
    takes_ownership(s);

    // integer makes its copy automatically
    let x = 5;
    makes_copy(x);

    let s2 = gives_ownership();

    let s3 = take_and_give_back(s2);

    let (s4, len) = return_string_and_length(s3);

    println!("The length of '{}' is {}", s4, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let s = String::from("hoge piyo");
    // if function returns a value, it move its ownership
    s
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}

fn return_string_and_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
