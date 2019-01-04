#[allow(unused_variables)]
pub fn example1() {
    println!("\nExample #1:");
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example2() {
    println!("\nExample #2:");
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example3() {
    println!("\nExample #3:");
    let s = String::from("hello");
    // These are equal:
    let slice = &s[0..2];
    let slice = &s[..2];
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example4() {
    println!("\nExample #4:");
    let s = String::from("hello");

    let len = s.len();
    // These are equal:
    let slice = &s[3..len];
    let slice = &s[3..];
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example5() {
    println!("\nExample #5:");
    let s = String::from("hello");

    let len = s.len();
    // These are equal:
    let slice = &s[0..len];
    let slice = &s[..];
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn example6() {
    println!("\nExample #6:");
    let mut s = String::from("hello world");

    let word = first_word_sliced(&s);

    //s.clear(); // Compile-time error: cannot borrow `s` as mutable because it is also borrowed as immutable
}


pub fn example7() {
    println!("\nExample #7:");
    let s = String::from("hello world");

    let word = first_word_sliced(&s);
    
    println!("First word of '{}' is '{}'", s, word);
}

fn first_word_sliced(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example8() {
    println!("\nExample #8:");
    let s = "Hello, world!"; // The type of s here is &str: 
                             // it’s a slice pointing to that specific point of the binary. 
                             // This is also why string literals are immutable; 
                             // &str is an immutable reference.
}

pub fn example9() {
    println!("\nExample #9:");
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_sliced_literal(&my_string[..]);
    println!("first_word works on slices of `String`s: {}", word);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_sliced_literal(&my_string_literal[..]);
    println!("first_word works on slices of string literals: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_sliced_literal(my_string_literal);
    println!("Because string literals *are* string slices already, this works too, without the slice syntax: {}", word);
}


fn first_word_sliced_literal(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn example10() {
    println!("\nExample #10:");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}