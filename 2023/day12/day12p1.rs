use std::convert::TryInto;
use std::fs;

fn find_unknown(springs: &Vec<char>) -> Option<usize> {
    for i in 0..springs.len() {
        if springs[i] == '?' {
            return Some(i);
        }
    }
    None
}

fn is_valid(springs: Vec<char>, groups: &Vec<u32>) -> u64 {
    let blocks = springs.iter().collect::<String>();
    let broken_blocks: Vec<&str> = blocks.split('.').filter(|x| !x.is_empty()).collect();

    if broken_blocks.len() != groups.len() {
        return 0;
    }

    for i in 0..broken_blocks.len() {
        if broken_blocks[i].len() != groups[i].try_into().unwrap() {
            return 0;
        }
    }

    return 1;
}

fn fill(springs: &mut Vec<char>, start: usize, size: usize, c: char) {
    for i in 0..start {
        if springs[i] == '?' {
            springs[i] = '.';
        }
    }
    for i in start..(start + size) {
        springs[i] = c;
    }
}

fn fill_broken(springs: &mut Vec<char>, start: usize, size: usize) {
    fill(springs, start, size, '#');
    if start + size < springs.len() && springs[start + size] == '?' {
        springs[start + size] = '.';
    }
}

fn place(springs: &Vec<char>, start: usize) -> Vec<Vec<char>> {
    let mut possible_springs: Vec<Vec<char>> = Vec::new();
    let max_size = springs
        .iter()
        .collect::<String>()
        .split(|x| x == '.' || x == '#')
        .filter(|x| !x.is_empty())
        .next()
        .unwrap()
        .len()
        + 1;

    for size in 1..max_size {
        for i in start..(start + max_size - size) {
            let mut new_springs = springs.clone();
            fill_broken(&mut new_springs, i, size);
            possible_springs.push(new_springs);
        }
    }

    let mut new_springs = springs.clone();
    fill(&mut new_springs, start, max_size - 1, '.');
    possible_springs.push(new_springs);

    possible_springs
}

fn get_combinations(springs: Vec<char>, groups: &Vec<u32>) -> u64 {
    let start = find_unknown(&springs);
    if start.is_none() {
        return is_valid(springs, groups);
    }

    let mut sum = 0;
    let start = start.unwrap();

    let next_springs = place(&springs, start);
    for s in next_springs {
        sum += get_combinations(s, groups);
    }

    sum
}

fn main() {
    let document = fs::read_to_string("day12.in").unwrap();
    let mut sum = 0;

    for line in document.lines() {
        let mut line = line.split_whitespace();
        let springs: Vec<char> = line.next().unwrap().chars().collect();
        let groups: Vec<u32> = line
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        sum += get_combinations(springs, &groups);
    }

    println!("{}", sum);
}
