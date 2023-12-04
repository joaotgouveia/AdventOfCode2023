use std::fs;

struct Point {
    x: usize,
    y: usize,
}

fn get_line(line: &str, start: usize, end: usize) -> String {
    let mut start = start;
    let mut end = end;

    let chars: Vec<char> = line.chars().collect();
    let max_index = line.len() - 1;

    while start > 0 && chars[start].is_numeric() {
        start -= 1;
    }

    while end < max_index && chars[end].is_numeric() {
        end += 1;
    }

    line[start..(end + 1)].chars().collect::<String>()
}

fn extract_square(schematic: &Vec<&str>, center: Point) -> Vec<String> {
    let line_end = schematic[0].len() - 1;
    let schematic_end = schematic.len() - 1;
    let x_start = if center.x > 0 { center.x - 1 } else { center.x };
    let y_start = if center.y > 0 { center.y - 1 } else { center.y };
    let x_end = if center.x < line_end {
        center.x + 1
    } else {
        center.x
    };
    let y_end = if center.y < schematic_end {
        center.y + 1
    } else {
        center.y
    };

    let mut lines: Vec<String> = Vec::new();
    for y in y_start..(y_end + 1) {
        lines.push(get_line(schematic[y], x_start, x_end));
    }

    lines
}

fn parse_numbers(line: &str) -> Vec<u32> {
    let line = line.split(|x: char| !x.is_numeric());
    let mut numbers: Vec<u32> = Vec::new();
    for symbol in line {
        let parsed = symbol.parse::<u32>();
        if parsed.is_ok() {
            numbers.push(parsed.unwrap());
        }
    }

    numbers
}

fn get_part_numbers(schematic: Vec<&str>) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = Vec::new();
    let mut squares: Vec<String> = Vec::new();

    for (y, line) in schematic.iter().enumerate() {
        for (x, digit) in line.chars().enumerate() {
            if !digit.is_numeric() && digit != '.' {
                squares.append(&mut extract_square(&schematic, Point { x, y }));
            }
        }
    }

    for line in squares {
        part_numbers.append(&mut parse_numbers(&line));
    }

    part_numbers
}

fn main() {
    let document = fs::read_to_string("day3.in").unwrap();
    let sum: u32 = get_part_numbers(document.lines().collect()).iter().sum();

    println!("{}", sum);
}
