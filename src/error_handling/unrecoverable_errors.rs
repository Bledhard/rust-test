fn example1() {    
    panic!("crash and burn"); // thread 'main' panicked at 'crash and burn'
}

fn example2() {
    let v = vec![1, 2, 3];

    v[99]; // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
}