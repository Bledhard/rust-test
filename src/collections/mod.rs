use std::collections::HashMap;
use std::io;

pub fn mean(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

pub fn mode(numbers: &[i32]) -> i32 {
    let mut occurrences = HashMap::new();

    for &value in numbers {
        *occurrences.entry(value).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers")
}

pub fn pig_latin() {
    println!("Enter the string to be Pig-latinized");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read from stdin");
    let mut chars = input.chars().peekable();
    let mut new_s = String::new();
    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                new_s.push(c);
                String::from("-hay")
            }
            'a'...'z' | 'A'...'Z' => {
                format!("-{}ay", c)
            }
            _ => {
                new_s.push(c);
                continue;
            }
        };

        while let Some(&c) = chars.peek() {
            match c {
                'a'...'z' | 'A'...'Z' => {
                    chars.next();
                    new_s.push(c);
                }
                _ => break,
            }
        }

        new_s += &suffix;
    }
    println!("{}", new_s);
}

mod directory;

pub fn dir_example() {
    directory::dir();
}