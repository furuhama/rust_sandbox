extern crate hello_world;

use hello_world::my_module;

fn main() {
    my_module::hello_rust();

    // my_module::guess_the_number();

    // my_module::fizzbuzz(35);

    // let n: i32 = 90;
    // println!("{}th fibonacci number: {}", n, my_module::get_fibonacci(n));
    // println!("{}th fibonacci number(tuple): {}", n, my_module::get_fibonacci_tuple(n));

    // my_module::run_basic_concepts();

    // my_module::basic_functions();

    // my_module::condition_controller();

    // my_module::practice_ownership();

    my_module::slice();
}
