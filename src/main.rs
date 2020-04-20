use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file.");

    println!("Contents:\n{}", contents);
}

#[allow(dead_code)]
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // &'static is the type of string literals, static program duration
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
