mod struct_basics;
mod struct_example;
mod method_syntax;

pub fn struct_examples() {
    println!("\nDefining and Instantiating Structs:");
    struct_basics::examples();
    println!("\nAn Example Program Using Structs:");
    struct_example::example();
    println!("\nMethod Syntax:");
    method_syntax::example();
}