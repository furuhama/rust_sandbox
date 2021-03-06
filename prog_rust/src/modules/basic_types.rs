// basic type

pub fn basic_types() {
    println!("{:?}", build_vec_verbose());
    println!("{:?}", build_vec());
    closure();
}

fn build_vec_verbose() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16); // means i16 type 10
    v.push(20_i16); // means i16 type 20
    v
}

fn build_vec() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

fn closure() {
    let is_even = |x: u64| -> bool { x % 2 == 0 };
    let d: u64 = 16;

    println!("{} is even?: {}", d, is_even(d));
}
