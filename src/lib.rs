mod collections;
mod generics;

use std::io;
use std::io::prelude::*;

pub fn startup() {
    let mut numbers = [42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

    println!("AVERAGE: {}", collections::mean(&numbers));
    println!("MEDIAN: {}", collections::median(&mut numbers));
    println!("MODE: {}", collections::mode(&numbers));

    collections::pig_latin();


    collections::dir_example();

    pause();
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}