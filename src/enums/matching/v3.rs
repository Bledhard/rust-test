#[allow(dead_code)]
#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}