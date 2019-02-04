use std::fs::File;
use std::io::prelude::Read;
use std::error::Error;
use super::config::Config;
use super::search::{search, search_case_insensitive};

pub fn read(config: &Config) -> Result<(), Box<Error>> {
    let mut f = File::open(&config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}