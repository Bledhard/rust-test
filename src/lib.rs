pub mod basics;
pub mod branches;
pub mod collections;
pub mod documentation;
pub mod enums;
pub mod error_handling;
pub mod functional_features;
pub mod generics;
pub mod minigrep;
pub mod modules;
pub mod ownership;
pub mod structs;

use std::io;
use std::io::prelude::*;

pub fn startup() {
    minigrep::startup();

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