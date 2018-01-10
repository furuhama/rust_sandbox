// use std::io;
extern crate hello_world;

use hello_world::my_module;

fn main() {
    println!("{}", my_module::hello_rust());

    // println!("please input some words");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("failed to read line");
    // println!("Input: {}", guess);

    println!("{}", my_module::fizzbuzz(35));

    println!("{}", my_module::get_fibonacci(90));
}

