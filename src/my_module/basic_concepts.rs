// practice Rust basic concepts

pub fn run_basic_concepts() {
    // to reassign something for one variable
    // use `mut`
    let mut x = 10;
    println!("{}", x);
    x = 100;
    println!("{}", x);

    // const needs type,
    // and it should be defined before computation
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // shadowing means redefine vars
    let piyo = 10;
    let piyo = piyo + 10;
    let piyo = piyo * 3;
    println!("{}", piyo);
}

