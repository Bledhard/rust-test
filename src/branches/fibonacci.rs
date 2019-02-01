pub fn fibonacci(count: i32) {
    let mut prev = 1;
    let mut curr = 1;
    let mut generated = 1;
    
    println!("{}", prev);
    while generated != count {
        println!("{}", curr);
        let _temp = prev;
        prev = curr;
        curr += _temp;
        generated += 1;
    }
}