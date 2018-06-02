extern crate adder;
extern crate hello_world;
extern crate prog_rust;

use hello_world::my_module;
use prog_rust::modules;

fn main() {
    my_module::hello_rust();

    // my_module::guess_the_number();

    // my_module::fizzbuzz(35);

    // my_module::fibonacci();

    // my_module::run_basic_concepts();

    // my_module::basic_functions();

    // my_module::condition_controller();

    // my_module::practice_ownership();

    // my_module::slice();

    // my_module::make_struct();

    // my_module::struct_example();

    // my_module::method_syntax();

    // my_module::define_enum();

    // my_module::match_control();

    // my_module::vector();

    // my_module::string();

    // my_module::hash_map();

    // my_module::error_handling();

    // my_module::generics();

    // my_module::lifetime();

    // my_module::minigrep();

    // my_module::functional_features();

    // my_module::pointers();

    // my_module::references_and_borrowings();

    // println!("square sum: {}", my_module::square_sum(10));

    // my_module::echo_server();

    // my_module::env_gcd();

    // modules::start_gcd_server();

    // modules::basic_types();

    // my_module::command();

    // my_module::state();

    // modules::table();

    my_module::run_libc();

    modules::references();

    my_module::pattern_matching();

    my_module::monkey_patch();
}
