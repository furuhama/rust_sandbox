// set modules in my_module

pub use self::fibonacci::get_fibonacci;
pub use self::fizzbuzz::fizzbuzz;
pub use self::hello_rust::hello_rust;

mod fibonacci;
mod fizzbuzz;
mod hello_rust;

