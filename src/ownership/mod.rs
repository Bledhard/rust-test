mod ownership_basics;
mod references_and_borrowing;
mod slices;

pub fn ownership_examples() {
    println!("\nOwnership  Basics:");
    ownership_basics::example1();
    ownership_basics::example2();
    ownership_basics::example3();

    println!("\nReferences and Borrrowing:");
    references_and_borrowing::example1();
    references_and_borrowing::example3();

    println!("\nSlices:");
    slices::example1();
    slices::example7();
    slices::example9();
}