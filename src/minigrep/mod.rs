mod file;
mod config;
mod search;

use self::config::Config;
use std::env;
use std::process;

pub fn startup() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = file::read(&config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}