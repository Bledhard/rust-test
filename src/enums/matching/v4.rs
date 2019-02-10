#[allow(dead_code)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example () {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}