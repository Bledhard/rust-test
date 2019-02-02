mod file;
mod config;

use self::config::Config;
use std::env;

pub fn start() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    let contents = file::read(&config.filename);

    println!("With text:\n{}", contents);
}