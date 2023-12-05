use std::fs;

struct Almanac {
    seeds: Vec<u64>,
    maps: [Vec<Vec<u64>>; 7],
}

impl Almanac {
    fn parse_almanac(lines: Vec<&str>) -> Self {
        let seeds = parse_line(lines[0]);
        let mut maps: [Vec<Vec<u64>>; 7] = Default::default();
        for i in 0..7 {
            maps[i] = parse_map(lines[i + 1]);
        }
        Self { seeds, maps }
    }
}

fn parse_line(line: &str) -> Vec<u64> {
    let line = line.split_whitespace();
    let mut nums: Vec<u64> = Vec::new();

    for num in line {
        nums.push(num.parse().unwrap());
    }

    nums
}

fn parse_map(nums: &str) -> Vec<Vec<u64>> {
    let nums = nums.lines();
    let mut map: Vec<Vec<u64>> = Vec::new();

    for line in nums {
        map.push(parse_line(line));
    }

    map
}

fn convert_number(num: u64, map: &Vec<Vec<u64>>) -> u64 {
    for line in map {
        if line[1] <= num && num < line[1] + line[2] {
            return num - line[1] + line[0];
        }
    }

    num
}

fn find_closest(maps: &[Vec<Vec<u64>>; 7], start: u64, end: u64) -> u64 {
    let mut min = 0;
    for map in maps {
        for seed in start..end {
            let location = convert_number(seed, map);
            if location < min || min == 0 {
                min = location;
            }
        }
    }

    min
}

fn main() {
    let document = fs::read_to_string("day5.in").unwrap();
    let maps: Vec<&str> = document
        .split(|x: char| !x.is_numeric() && x != ' ' && x != '\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();
    let almanac = Almanac::parse_almanac(maps);
    let ranges = almanac.seeds;
    let range_count = ranges.len() / 2;

    let mut locations: Vec<u64> = Vec::new();
    for i in 0..range_count {
        let start = ranges[2 * i];
        let end = start + ranges[2 * i + 1];
        locations.push(find_closest(&almanac.maps, start, end));
    }

    println!("{}", locations.iter().min().unwrap());
}
