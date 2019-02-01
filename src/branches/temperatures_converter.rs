pub fn convert_temperature(degrees: i32, scale: &str) {
    if scale == "F" {
        let converted = to_celsius(degrees);    
        println!("{} F is equal to {} C", degrees, converted);
    }
    else {
        let converted = to_fahrenheit(degrees);
        println!("{} C is equal to {} F", degrees, converted);
    };
}

fn to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5/9
}

fn to_fahrenheit(celsius: i32) -> i32 {
    (celsius * 9/5) + 32
}