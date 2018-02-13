// implement mini grep command

use std::env;

pub fn minigrep() {
    // just read arguments and return them
    let args: Vec<String> = env::args().collect();

    // args[0] == "target/debug/hello_world"
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for: {}", query);
    println!("In file: {}", filename);
}
