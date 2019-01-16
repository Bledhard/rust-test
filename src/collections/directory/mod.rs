mod parser;
mod commands;
mod dir_struct;

use std::io;
use std::collections::HashMap;
use self::parser::parse_input;
use self::commands::ExitCode;
use collections::directory::dir_struct::Directory;

pub fn dir() {    
    let mut directory = Directory {
        departments: HashMap::new(),
    };

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let command = parse_input(input);
        let (exit_code, message) = command.execute(&mut directory);

        print!("{}", message);

        if let ExitCode::Exit = exit_code {
            break;
        }
    }
}