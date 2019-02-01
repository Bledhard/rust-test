pub fn function_examples() {
    example1(5);
    example2(5, 6);
    example3();
    example4();
    example5();
}

fn example1(x: i32) {
    println!("\nExample #1:");
    println!("The value of x is {}", x);
}

fn example2(x: i32, y: i32) {
    println!("\nExample #2:");
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn example3() {
    println!("\nExample #3:");    
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of the outer x is: {}", x);
}

fn example4() {
    println!("\nExample #4:");  
    let x = five();

    println!("The value of x is {}", x);
}

fn example5() {
    println!("\nExample #5:");  
    let x = explicit_five();

    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn explicit_five() -> i32 {
    return 5;
}