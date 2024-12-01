use std::fs;

fn hash(string: &str) -> u64 {
    let mut hashed_string = 0;
    for c in string.chars() {
        hashed_string += c as u64;
        hashed_string *= 17;
        hashed_string %= 256;
    }

    hashed_string
}

fn main() {
    let document = fs::read_to_string("day15.in").unwrap();
    let initialization_sequence = document.split(",").collect::<Vec<&str>>();

    let mut sum = 0;
    for step in initialization_sequence {
        sum += hash(step);
    }

    println!("{}", sum);
}
