use std::fs;

fn parse_line(line: &str) -> u32 {
    let mut first_digit = 0;
    for digit in line.chars() {
        if digit.is_digit(10) {
            first_digit = digit.to_digit(10).expect("Something went wrong");
            break;
        }
    }

    let mut last_digit = 0;
    for digit in line.chars().rev() {
        if digit.is_digit(10) {
            last_digit = digit.to_digit(10).expect("Something went wrong");
            break;
        }
    }

    first_digit * 10 + last_digit
}

fn main() {
    let mut sum = 0;
    let document = fs::read_to_string("day1.in").expect("Failed to read input");

    for line in document.lines() {
        sum += parse_line(line);
    }

    println!("{}", sum);
}
