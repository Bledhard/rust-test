const MAX_POINTS: u32 = 100_000;

pub fn variables() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
}

fn example1() {
    println!("\nExample #1:");
    let mut x = 5; // will give error without 'mut' keyword
    // let x = 5; - compile-time error
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn example2() {
    println!("\nExample #2:");
    println!("The value of constant 'MAX_POINTS' is: {}", MAX_POINTS);
}

fn example3() { 
    println!("\nExample #3:");
    let x = 5;    
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn example4() {
    println!("\nExample #4:");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn example5() {
    println!("\nExample #5:");
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn example6() {
    println!("\nExample #6:");

    let a = [1, 2, 3, 4, 5];
    let x = a[0]; // 1
    let y = a[1]; // 2
    let index = 10;

    let element = a[index];

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of element is: {}", element); // Runtime error - index out of bounds
}