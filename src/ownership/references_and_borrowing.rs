pub fn example1() {
    println!("\nExample #1:");
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// Here is how you would define and use a calculate_length function 
// that has a reference to an object as a parameter 
// instead of taking ownership of the value:
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// Example of attempt to modify borrowed variable
#[allow(dead_code)]
pub fn example2() {
    println!("\nExample #2:");
    let s = String::from("hello");

    change(&s);
}

#[allow(unused_variables)]
fn change(some_string: &String) {
    //some_string.push_str(", world"); // Compile-time error: cannot borrow immutable borrowed content `*some_string` as mutable
}

pub fn example3() {
    println!("\nExample #3:");
    let mut s = String::from("hello");

    change_mutable(&mut s);
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world");
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example4() {
    println!("\nExample #4:");
    let mut s = String::from("hello");

    // Mutable references have one big restriction: 
    // you can have only one mutable reference 
    // to a particular piece of data in a particular scope. 
    // This code will fail:
    //let r1 = &mut s;
    //let r2 = &mut s; // Compile-time error: cannot borrow `s` as mutable more than once at a time

    // But this one works:
    {
        let r1 = &mut s;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn example5() {
    println!("\nExample #5:");
    let mut s = String::from("hello");

    // A similar rule exists for combining mutable and immutable references. 
    // This code results in an error:
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // Compile-time error: cannot borrow `s` as mutable because it is also borrowed as immutable

}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn example6() {
    println!("\nExample #6:");
    //let reference_to_nothing = dangle();
    let reference_to_something = no_dangle();
}

//fn dangle() -> &String { // Compile-time error: missing lifetime specifier
                           // dangle returns a reference to a String
//    let s = String::from("hello"); // s is a new String
//
//    &s // we return a reference to the String, s
//} // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!

fn no_dangle() -> String {
let s = String::from("hello");

s
}