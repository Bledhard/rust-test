use super::user_1::User;

pub fn example1() {
    println!("\nExample #1:");
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {}", user1);
}

pub fn example2() {
    println!("\nExample #2:");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {}", user1);

    user1.email = String::from("anotheremail@example.com");

    println!("user1: {}", user1);
}

pub fn example3() {
    println!("\nExample #3:");

    let user1 = build_user(String::from("anotheremail@example.com"), String::from("someusername123"));

    println!("user1: {}", user1);
}

pub fn example4() {
    println!("\nExample #4:");

    let user1 = build_user_shorthand(String::from("anotheremail@example.com"), String::from("someusername123"));

    println!("user1: {}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// as long as field name and parameter name are same, we can use shorthand syntax
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn example5() {
    println!("\nExample #5:");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("user2: {}", user2);
}

pub fn example6() {
    println!("\nExample #6:");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1: {}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("user2: {}", user2);
}

pub fn example7() {
    println!("\nExample #7:");

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
}