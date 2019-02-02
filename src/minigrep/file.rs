use std::fs::File;
use std::io::prelude::Read;
use std::error::Error;

pub fn read(filename: &str) -> Result<(), Box<Error>> {  
    println!("In file {}", filename); 

    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}