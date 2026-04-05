use std::error::Error;
use std::{env, fs, process::exit};
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    println!("{}", contents);
    Ok(())
}
pub struct Config {
    pub query: String,

    pub file: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            print!("Not Enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query,
            file: file_path,
        })
    }
}
