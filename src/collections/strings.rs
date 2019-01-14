pub fn examples() {

}

fn example1() {
    let mut s = String::new();
}

fn example2() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}

fn example3() {
    let s = String::from("initial contents");
}

fn example4() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn example5() {
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn example6() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}

fn example7() {
    let mut s = String::from("lo");
    s.push('l');
}

fn example8() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

fn example9() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // tic-tac-toe
}

fn example10() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn example11() {
    let s1 = String::from("hello");
    //let h = s1[0]; // ERROR: the type `std::string::String` cannot be indexed by `{integer}`
}

fn example12() {
    let len = String::from("Hola").len(); // 4
    let len = String::from("Здравствуйте").len(); // 24, not 12
}

fn example13() {
    let s1 = "नमस्ते";
    // s1 is a vector:
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] 
    // in chars:
    // ["न", "म", "स्", "ते"]
}

fn example14() {
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // "Зд", because each char is 2 byte long
    // let s = &hello[0..1] // thread panick - half of a char
}

fn example15() {
    for c in "नमस्ते".chars() {
        println!("{}", c); // ["न", "म", "स्", "ते"]
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b); // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] 
    }
}