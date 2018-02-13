// implement mini grep command

use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn minigrep() {
    // just read arguments and return them
    let args: Vec<String> = env::args().collect();

    // args[0] == "target/debug/hello_world"
    let (query, filename) = parse_config(&args);

    println!("Searching for: {}", query);
    println!("In file: {}", filename);

    // try to open file which name is given as filename
    let mut f = File::open(filename).expect("file not found");

    // try to set content from read file into contents
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file.");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
