use std::fs::File;
use std::io::prelude::Read;

pub fn read(filename: &String) -> String {
    println!("In file {}", filename);    

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}