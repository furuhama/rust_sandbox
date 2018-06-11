use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

pub fn minigrep() {
    // just read arguments and return them
    // let args: Vec<String> = env::args().collect();

    // args[0] == "target/debug/hello_world"
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //
    //     process::exit(1);
    // });

    // ======================================================

    // just set env::args() as argument
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // old implementation

    // let mut results = Vec::new();

    // iterate contents for each line,
    // if it contains query, its line will be pushed to results vec.
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    // ================================================================

    // use iterator

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// extract logic fron main function
fn run(config: Config) -> Result<(), Box<Error>> {
    // try to open file which name is given as filename
    let mut f = File::open(config.filename)?;

    // try to set content from read file into contents
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Results:");

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // error handling, when the number of arguments is less than expected
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // not to use clone() method (it is sometimes expensive process)
        // let query = args[1].clone();
        // let filename = args[2].clone();

        // skip first argument, since it is something like a turd for grepping
        let _ = args.next();

        let query = match args.next() {
            Some(arg) => {
                println!("Search Query: {}", arg);
                arg
            },
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => {
                println!("File Path: {}", arg);
                arg
            },
            None => return Err("Didn't get a file name"),
        };

        // read env vars
        // CASE_INSENSITIVE=1 cargo run [query] [contents]
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
