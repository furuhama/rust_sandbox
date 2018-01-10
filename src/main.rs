// use std::io;
extern crate hello_world;

use hello_world::my_module;

fn main() {
    my_module::hello_rust();

    // println!("please input some words");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("failed to read line");
    // println!("Input: {}", guess);

    my_module::fizzbuzz(35);

    let n: i32 = 90;
    println!("{}th fibonacci number: {}", n, my_module::get_fibonacci(n));
}

