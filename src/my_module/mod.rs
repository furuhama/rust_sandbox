// set modules in my_module

// redefine functions as be in this file
pub use self::fibonacci::fibonacci;
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
// comment out modules below('cause their warning messages are noisy)
// pub use self::define_enum::define_enum;
// pub use self::match_control::match_control;
pub use self::vector::vector;
pub use self::string::string;
pub use self::hash_map::hash_map;
pub use self::error_handling::error_handling;
// pub use self::generics::generics;
// pub use self::lifetime::lifetime;
pub use self::minigrep::minigrep;
pub use self::minigrep::search;
pub use self::minigrep::search_case_insensitive;
pub use self::functional_features::functional_features;
pub use self::functional_features::Cacher;
pub use self::functional_features::_Counter;
pub use self::functional_features::square_sum;
pub use self::pointer::pointers;
pub use self::pointer::Messenger;
pub use self::pointer::LimitTracker;
pub use self::first_edition_japanese::references_and_borrowings;
pub use self::rust_as_a_second_lang::echo_server;
pub use self::prog_rust::env_gcd;

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
// mod define_enum;
// mod match_control;
mod vector;
mod string;
mod hash_map;
mod error_handling;
// mod generics;
// mod lifetime;
mod minigrep;
mod functional_features;
mod pointer;
mod first_edition_japanese;
mod rust_as_a_second_lang;
mod prog_rust;
