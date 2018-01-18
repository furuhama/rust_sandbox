extern crate hello_world;

use hello_world::my_module;

fn main() {
    my_module::hello_rust();

    // my_module::guess_the_number();

    // my_module::fizzbuzz(35);

    // let n: i32 = 90;
    // println!("{}th fibonacci number: {}", n, my_module::get_fibonacci(n));

    let mut x = 5;
    println!("{}", x);
    x = 10;
    println!("{}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    let hoge = 10;
    let hoge = hoge * 2;
    let hoge = hoge + 12;
    println!("{}", hoge);
}

