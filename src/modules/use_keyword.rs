pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

pub fn example() {
    a::series::of::nested_modules();
}

use a::series::of:;

pub fn use_example() {
    of::nested_modules();
}

use a::series::of::nested_modules;

pub fn use_fn_example() {
    nested_modules();
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn use_enum_example() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}