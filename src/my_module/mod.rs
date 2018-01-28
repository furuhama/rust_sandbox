// set modules in my_module

// redefine functions as be in this file
pub use self::fibonacci::get_fibonacci;
pub use self::fibonacci::get_fibonacci_tuple;
pub use self::fizzbuzz::fizzbuzz;
pub use self::hello_rust::hello_rust;
pub use self::get_input_and_return::guess_the_number;
pub use self::basic_concepts::run_basic_concepts;
pub use self::basic_functions::basic_functions;
pub use self::condition_control::condition_controller;
pub use self::ownership::practice_ownership;
pub use self::slice::slice;
pub use self::make_struct::make_struct;
pub use self::struct_example::struct_example;
pub use self::method_syntax::method_syntax;
pub use self::define_enum::define_enum;
pub use self::match_control::match_control;

// call mod files
mod fibonacci;
mod fizzbuzz;
mod hello_rust;
mod get_input_and_return;
mod basic_concepts;
mod basic_functions;
mod condition_control;
mod ownership;
mod slice;
mod make_struct;
mod struct_example;
mod method_syntax;
mod define_enum;
mod match_control;
