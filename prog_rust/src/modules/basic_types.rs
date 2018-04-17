pub fn basic_types() {
    println!("{:?}", build_vec_verbose());
    println!("{:?}", build_vec());
}

fn build_vec_verbose() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vec() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}
