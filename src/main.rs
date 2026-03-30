use std::error::Error;
use std::{env, fs, process::exit};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {err}");
        exit(1);
    });
    println!("searching for {} in file {}", config.query, config.file);
    if let Err(e) = run(config) {
        println!("Error : {e}");
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    println!("{}", contents);
    Ok(())
}
struct Config {
    query: String,
    file: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
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
