#[allow(dead_code)]
pub fn branching_examples() {
    example1();
    example2();
    example3();
    example4();
    example6();
    example7();
    example8();
    example9();
}

fn example1() {
    println!("\nExample #1:");
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// Complete copy of example1(), but written in C# style
// Provides the same result
#[allow(unused_parens)] // to disable 'remove these paretheses' warning
fn example2() 
{
    println!("\nExample #2:");
    let number = 5;

    if (number < 5)
    {
        println!("condition was true");
    }
    else
    {
        println!("condition was false");
    }
}

fn example3() {
    println!("\nExample #3:");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn example4() {
    println!("\nExample #4:");
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

#[allow(dead_code)]
fn example5() {
    println!("\nExample #5:");
    // Infinite loop
    // The loop keyword tells Rust to execute a block of code over and over again forever 
    // or until you explicitly tell it to stop.
    loop {
        println!("again!");
    }
}

fn example6() {
    println!("\nExample #6:");
    let mut count = 1;

    loop {
        println!("again!");
        count += 1;
        if count > 5 {
            break;
        }
    }
}

fn example7() {
    println!("\nExample #7:");
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn example8() {
    println!("\nExample #8:");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn example9() {
    println!("\nExample #9:");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}