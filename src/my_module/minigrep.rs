// implement mini grep command

use std::env;

pub fn minigrep() {
    // just read arguments and return them
    let args: Vec<String> = env::args().collect();

    let index_zero = &args[0];
    let query = &args[1];
    let filename = &args[2];

    println!("Zero index: {}", index_zero);
    println!("Searching for: {}", query);
    println!("In file: {}", filename);
}
