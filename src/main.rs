use minigrep::{Config, run};
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
