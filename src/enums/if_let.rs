#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn example_if() {
    let some_u8_value = Some(0u8);

    if let Some(3) = some_u8_value {
        println!("three");
    }
}