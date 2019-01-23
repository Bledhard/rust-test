struct Point<T> {
    x: T,
    y: T,
}

fn example1() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

fn example2() {
    //let wont_work = Point { x: 5, y: 4.0 };

    //     error[E0308]: mismatched types
    //  --> src/main.rs:7:38
    //   |
    // 7 |     let wont_work = Point { x: 5, y: 4.0 };
    //   |                                      ^^^ expected integral variable, found
    // floating-point variable
    //   |
    //   = note: expected type `{integer}`
    //              found type `{float}`
}

fn example3() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}