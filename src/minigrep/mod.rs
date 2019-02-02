mod file;
mod config;

use self::config::Config;
use std::env;
use std::process;

pub fn startup() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);

    if let Err(e) = file::read(&config.filename) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}