use std::fs;

fn swap_spelled_number(spelled_number: &str) -> u32 {
    if spelled_number.contains("one") {
        1
    } else if spelled_number.contains("two") {
        2
    } else if spelled_number.contains("three") {
        3
    } else if spelled_number.contains("four") {
        4
    } else if spelled_number.contains("five") {
        5
    } else if spelled_number.contains("six") {
        6
    } else if spelled_number.contains("seven") {
        7
    } else if spelled_number.contains("eight") {
        8
    } else if spelled_number.contains("nine") {
        9
    } else {
        0
    }
}

fn parse_line(line: &str) -> u32 {
    let mut spelled_number = String::new();
    let mut fixed_number: u32;

    let mut first_digit = 0;
    for digit in line.chars() {
        if digit.is_digit(10) {
            first_digit = digit.to_digit(10).expect("Something went wrong");
            break;
        } else {
            spelled_number.push(digit);
            fixed_number = swap_spelled_number(&spelled_number);
            if fixed_number != 0 {
                first_digit = fixed_number;
                break;
            }
        }
    }

    spelled_number.clear();

    let mut last_digit = 0;
    for digit in line.chars().rev() {
        if digit.is_digit(10) {
            last_digit = digit.to_digit(10).expect("Something went wrong");
            break;
        } else {
            spelled_number.push(digit);
            fixed_number = swap_spelled_number(&spelled_number.chars().rev().collect::<String>());
            if fixed_number != 0 {
                last_digit = fixed_number;
                break;
            }
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
