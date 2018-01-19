// set modules in my_module

// redefine functions as be in this file
pub use self::fibonacci::get_fibonacci;
pub use self::fibonacci::get_fibonacci_tuple;
pub use self::fizzbuzz::fizzbuzz;
pub use self::hello_rust::hello_rust;
pub use self::get_input_and_return::guess_the_number;
pub use self::basic_concepts::run_basic_concepts;
pub use self::basic_functions::basic_functions;

// call mod files
mod fibonacci;
mod fizzbuzz;
mod hello_rust;
mod get_input_and_return;
mod basic_concepts;
mod basic_functions;

