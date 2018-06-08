// practice ownership

pub fn practice_ownership() {
    // use String::from, we can combine String
    let mut s = String::from("hello");

    s.push_str(", rust!");

    // String's ownership is moved to this function
    takes_ownership(s);

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
    let length = use_reference(&s5);

    println!("The length of '{}' is {}", s5, length);

    // Mutable & borrowing
    // this function changes String value
    // without moving its ownership (just by mutable borrowing)
    let mut s6 = String::from("hello");
    change_str(&mut s6);
    println!("{}", s6);

    let v: Vec<u64> = vec![1, 2, 3, 4];
    for elem in &v {
        ref_print(*elem);
    }

    fn ref_print(num: u64) {
        println!("{}", num);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let s = String::from("hoge piyo");
    // when this function returns a value,
    // the ownership of the value also moves to outside this function
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

fn change_str(s: &mut String) {
    s.push_str(", Rust!!!");
}
