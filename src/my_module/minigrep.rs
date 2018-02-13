// implement mini grep command

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

pub fn minigrep() {
    // just read arguments and return them
    let args: Vec<String> = env::args().collect();

    // args[0] == "target/debug/hello_world"
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// extract logic fron main function
fn run(config: Config) -> Result<(), Box<Error>> {
    // try to open file which name is given as filename
    let mut f = File::open(config.filename)?;

    // try to set content from read file into contents
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // error handling, when the number of arguments is less than expected
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
