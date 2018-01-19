// practice Rust basic concepts

pub fn run_basic_concepts() {
    // Vatiables and Mutability

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


    // Types

    // type annotation
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // character type
    let emoji = '😻';
    println!("cutest emoji: {}", emoji);

    // Grouping values into tuples
    let tup: (i32, i32, i32) = (10, 50, 64);
    let (tup_1, tup_2, tup_3) = tup;
    println!("tuple values: {}, {}, {}", tup_1, tup_2, tup_3);
}

