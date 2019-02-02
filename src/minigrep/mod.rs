mod file_read;

use std::env;

pub fn start() {
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    let contents = file_read::read(&filename);

    println!("With text:\n{}", contents);
}