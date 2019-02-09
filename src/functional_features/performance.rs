let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])       // What assembly code would this Rust code compile to?
                                 .map(|(&c, &s)| c * s as i64)  // Well, as of this writing, it compiles down to the same assembly
                                 .sum::<i64>() >> qlp_shift;    // you’d write by hand. There’s no loop at all corresponding to the iteration
                                                                // over the values in coefficients: Rust knows that there are 12 iterations, 
                                                                // so it “unrolls” the loop. Unrolling is an optimization that removes 
                                                                // the overhead of the loop controlling code and instead generates repetitive 
                                                                // code for each iteration of the loop.
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}

// We ran a benchmark by loading the entire contents of 
// The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle 
// into a String and looking for the word the in the contents. 
// Here are the results of the benchmark on the version of search 
// using the for loop and the version using iterators:

// test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
// test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

// The point is this: iterators, although a high-level abstraction, 
// get compiled down to roughly the same code as if you’d written the lower-level code yourself. 
// Iterators are one of Rust’s zero-cost abstractions,
// by which we mean using the abstraction imposes no additional runtime overhead.