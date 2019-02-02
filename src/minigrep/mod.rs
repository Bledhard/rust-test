mod file;
mod config;

use self::config::Config;
use std::env;
use std::process;
use std::error::Error;

pub fn startup() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename); 

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let contents = file::read(&config.filename);

    println!("With text:\n{}", contents);

    Ok(())
}