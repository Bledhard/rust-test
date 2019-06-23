pub mod rcmemoryleak;

#[allow(dead_code)]
fn example1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

pub fn main() {
    rcmemoryleak::main();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn test_list() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}