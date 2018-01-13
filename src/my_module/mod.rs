// set modules in my_module

pub use self::fibonacci::get_fibonacci;
pub use self::fizzbuzz::fizzbuzz;
pub use self::hello_rust::hello_rust;
pub use self::get_input_and_return::get_and_print_input;
pub use self::get_input_and_return::guess_the_number;

mod fibonacci;
mod fizzbuzz;
mod hello_rust;
mod get_input_and_return;

