mod method_1;
mod method_2;
mod associated_fn;
mod multi_impl;

pub fn example() {
    println!("\nDefining Methods:");
    method_1::example();
    println!("\nMethods with More Parameters:");
    method_2::example();
    println!("\nAssociated functions:");
    associated_fn::example();
    println!("\nMultiple impl Blocks:");
    multi_impl::example();
}