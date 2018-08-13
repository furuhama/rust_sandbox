extern crate hello_world;
extern crate adder;
extern crate prog_rust;
extern crate minigrep;

#[allow(unused_imports)]
use hello_world::my_module;
#[allow(unused_imports)]
use prog_rust::modules;
#[allow(unused_imports)]
use minigrep::minimodule;

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

    // minimodule::minigrep();

    // my_module::functional_features();

    // my_module::pointers();

    // my_module::references_and_borrowings();

    // my_module::echo_server();

    // modules::env_gcd();

    // modules::start_gcd_server();

    // modules::basic_types();

    // my_module::command();

    // my_module::state();

    // modules::table();

    // my_module::run_libc();

    // modules::references();

    // my_module::pattern_matching();

    // my_module::monkey_patch();

    // my_module::try_macro();

    // my_module::existential();

    // my_module::concurrent();

    // my_module::concurrent_with_channel();

    modules::queue();

    my_module::file_io();
}
