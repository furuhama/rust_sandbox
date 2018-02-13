// implement mini grep command

use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn minigrep() {
    // just read arguments and return them
    let args: Vec<String> = env::args().collect();

    // args[0] == "target/debug/hello_world"
    let config = Config::new(&args);

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // try to open file which name is given as filename
    let mut f = File::open(config.filename).expect("file not found");

    // try to set content from read file into contents
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file.");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
