use std::fs::File;
use std::io;
use std::io::Read;

pub fn example1() {
    let f = File::open("hello.txt");
    let f: u32 = File::open("hello.txt"); 
    // error[E0308]: mismatched types
    //   |
    // 5 |     let f: u32 = File::open("hello.txt");
    //   |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
    // `std::result::Result`
}

pub fn example2() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}

pub fn example3() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}

pub fn example4() {
    let f = File::open("hello.txt").unwrap();
}

pub fn example5() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shortcut_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

pub fn example6() {
    let f = File::open("hello.txt")?; // error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
    //    |
    // 91 |     let f = File::open("hello.txt")?;
    //    |             ------------------------
    //    |             |
    //    |             the `?` operator can only be used in a function that returns
    //    `Result` (or another type that implements `std::ops::Try`)
    //    |             in this macro invocation
    //    |
    //    = help: the trait `std::ops::Try` is not implemented for `()`
    //    = note: required by `std::ops::Try::from_error`
}