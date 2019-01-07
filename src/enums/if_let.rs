pub fn example_match() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

pub fn example_if() {
    if let Some(3) = some_u8_value {
        println!("three");
    }
}