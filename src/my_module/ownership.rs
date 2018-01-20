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

    // from inside function, ownership moved
    let s2 = gives_ownership();

    // move ownership from s2 to s3
    let s3 = take_and_give_back(s2);

    // to calculate length
    // Pattern 1: move String ownership to function once,
    // and return String & usize as a tuple
    let (s4, len) = return_string_and_length(s3);

    println!("The length of '{}' is {}", s4, len);

    // Pattern 2: Use reference (it is called `bollowing` in Rust)
    let s5 = gives_ownership();
    let length = use_reference(&s5);

    println!("The length of '{}' is {}", s5, length);
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

fn use_reference(s: &String) -> usize {
    s.len()
}
