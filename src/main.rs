use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file.");

    println!("Contents:\n{}", contents);
}

#[allow(dead_code)]
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
