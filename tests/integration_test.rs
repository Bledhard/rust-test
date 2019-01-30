extern crate hello_world;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, hello_world::add_two(2));
}
