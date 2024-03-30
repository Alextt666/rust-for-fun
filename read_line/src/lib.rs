use std::error::Error;
use std::fs;
pub struct Config {
    pub query_for: String,
    pub target_path: String,
}
impl Config {
   pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }
        let query_for = args[1].clone();
        let target_path = args[2].clone();
        Ok(Config {
            query_for,
            target_path,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.target_path)?;
    println!("query_for {}\n{}", config.query_for, content);
    Ok(())
}
