extern crate hello_world;
extern crate num;

use hello_world::my_module;

fn main() {
    my_module::hello_rust();

    my_module::get_and_print_input();

    my_module::fizzbuzz(35);

    let n: i32 = 90;
    println!("{}th fibonacci number: {}", n, my_module::get_fibonacci(n));
}

