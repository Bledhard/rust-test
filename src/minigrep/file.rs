use std::fs::File;
use std::io::prelude::Read;
use std::error::Error;
use super::{config::Config, search};

pub fn read(config: &Config) -> Result<(), Box<Error>> {
    let mut f = File::open(&config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}