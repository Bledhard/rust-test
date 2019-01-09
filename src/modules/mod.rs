mod other_file;
mod separated_parent;
mod super_keyword;

pub fn example() {
    network::connect();
    client::connect();

    parent::connect();
    parent::nested::connect();

    other_file::connect();

    separated_parent::connect();
    separated_parent::server::connect();
}

mod network {
    pub fn connect() {

    }
}

mod client {
    pub fn connect() {

    }
}

mod parent {
    pub fn connect() {

    }

    pub mod nested {
        pub fn connect() {

        }
    }
}

// mod outermost {
//     pub fn middle_function() {}

//     fn middle_secret_function() {}

//     mod inside {
//         pub fn inner_function() { }

//         fn secret_function() {}
//     }

//     pub mod call_external {
//         pub fn function() {
//             ::modules::outermost::middle_secret_function(); // works just fine
//         }
//     }
// }

// fn try_me() {
//     outermost::middle_function();
//     outermost::middle_secret_function();  // err
//     outermost::inside::inner_function();  // err
//     outermost::inside::secret_function(); // err
// }

#[allow(dead_code)]
pub fn super_example() {
    super_keyword::example();
}