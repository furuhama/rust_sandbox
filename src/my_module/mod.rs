// set modules in my_module

// redefine functions as it is in this file
pub use self::basic_concepts::run_basic_concepts;
pub use self::basic_functions::basic_functions;
pub use self::condition_control::condition_controller;
pub use self::fibonacci::fibonacci;
pub use self::fizzbuzz::fizzbuzz;
pub use self::get_input_and_return::guess_the_number;
pub use self::hello_rust::hello_rust;
pub use self::make_struct::make_struct;
pub use self::method_syntax::method_syntax;
pub use self::ownership::practice_ownership;
pub use self::slice::slice;
pub use self::struct_example::struct_example;
// comment out modules below('cause their warning messages are noisy)
// pub use self::define_enum::define_enum;
// pub use self::match_control::match_control;
pub use self::error_handling::error_handling;
pub use self::hash_map::hash_map;
pub use self::string::string;
pub use self::vector::vector;
// pub use self::generics::generics;
// pub use self::lifetime::lifetime;
pub use self::first_edition_japanese::references_and_borrowings;
pub use self::functional_features::functional_features;
pub use self::functional_features::Cacher;
pub use self::functional_features::_Counter;
pub use self::functional_features::square_sum;
pub use self::minigrep::minigrep;
pub use self::minigrep::search;
pub use self::minigrep::search_case_insensitive;
pub use self::pointer::pointers;
pub use self::pointer::LimitTracker;
pub use self::pointer::Messenger;
pub use self::rust_as_a_second_lang::echo_server;
pub use self::design_pattern::command;
pub use self::design_pattern::state;
pub use self::libc::run_libc;

// call mod files
mod basic_concepts;
mod basic_functions;
mod condition_control;
mod fibonacci;
mod fizzbuzz;
mod get_input_and_return;
mod hello_rust;
mod make_struct;
mod method_syntax;
mod ownership;
mod slice;
mod struct_example;
// mod define_enum;
// mod match_control;
mod error_handling;
mod hash_map;
mod string;
mod vector;
// mod generics;
// mod lifetime;
mod first_edition_japanese;
mod functional_features;
mod minigrep;
mod pointer;
mod rust_as_a_second_lang;
mod design_pattern;
mod libc;
