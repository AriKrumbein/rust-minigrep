use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // &'static is the type of string literals, static program duration
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents =
      fs::read_to_string(config.filename)?; // question mark is shorthand for propogate error up
  
  println!("Contents:\n{}", contents);
  Ok(())
}