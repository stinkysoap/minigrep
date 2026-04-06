use minigrep::{Config, run};
use std::{env, fs, process::exit};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {err}");
        exit(1);
    });
    println!("searching for {} in file {}", config.query, config.file);
    if let Err(e) = run(config) {
        eprintln!("Error : {e}");
    }
}
