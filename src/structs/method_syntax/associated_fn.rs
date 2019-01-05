#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

pub fn example() {
    let sq = Rectangle::square(3);

    println!(
        "Associated function 'square' created Rectangle: {:#?}", 
        sq
    );
}