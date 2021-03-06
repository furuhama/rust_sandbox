// practice ownership

pub fn practice_ownership() {
    // use String::from, we can combine String
    let mut s = String::from("hello");

    s.push_str(", rust!");

    // String's ownership is moved to this function
    takes_ownership(&s);

    // integer makes its copy automatically,
    // because integer (like i32) types have Copy trait
    // (having Copy trait means its value is stored on stack memory)
    let x = 5;
    makes_copy(x);

    // ownership moves from inside this function to local variable s2
    let s2 = gives_ownership();

    // ownership moves from s2 to s3
    let s3 = take_and_give_back(s2);

    // To calculate length

    // Pattern 1: String ownership moves from s3 to function once,
    // and it returns to s4 and len as a tuple consisting of String and usize
    // (NOTE: we can set some variables at once by using tuple like this)
    let (s4, len) = return_string_and_length(s3);

    println!("The length of '{}' is {}", s4, len);

    // Pattern 2: Use reference (it is called `borrowing` in Rust)
    let s5 = gives_ownership();
    let length = use_reference_to_calculate_length(&s5);

    println!("The length of '{}' is {}", s5, length);

    // Mutable & borrowing
    // this function changes String value
    // without moving its ownership (just by mutable borrowing)
    let mut s6 = String::from("Hello");
    change_string(&mut s6);
    println!("{}", s6);

    vec_borrowing();

    string_borrowing_twice();
}

fn takes_ownership(some_string: &str) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    // when this function returns a value,
    // the ownership of the value also moves to outside this function
    String::from("hoge piyo")
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}

fn return_string_and_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn use_reference_to_calculate_length(s: &str) -> usize {
    // just referencing String type value
    // it is called `borrowing` to use reference from functions
    s.len()
}

fn change_string(s: &mut String) {
    // mutable referencing String value
    // it should be just one mutable reference for each value at highest
    // since it would cause value confliction by two or more mutable references
    s.push_str(", Rust!!!");
}

fn vec_borrowing() {
    let v: Vec<u64> = vec![1, 2, 3, 4];
    for elem in &v {
        ref_print(*elem);
    }

    fn ref_print(num: u64) {
        println!("{}", num);
    }
}

fn string_borrowing_twice() {
    let mut s = String::from("hoge");

    {
        let r1 = &mut s;
        r1.push_str(" fuga");
    }

    let r2 = &mut s;
    r2.push_str(" piyo");
    println!("{}", r2);
}
