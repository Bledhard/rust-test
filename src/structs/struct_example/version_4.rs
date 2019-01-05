#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn example() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);

    println!("rect1 is {:#?}", rect1);
}