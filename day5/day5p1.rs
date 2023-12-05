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

fn find_closest(almanac: Almanac) -> u64 {
    let seed_count = almanac.seeds.len();
    let mut locations: Vec<u64> = almanac.seeds;

    for map in almanac.maps {
        for i in 0..seed_count {
            locations[i] = convert_number(locations[i], &map)
        }
    }

    *locations.iter().min().unwrap()
}

fn main() {
    let document = fs::read_to_string("day5.in").unwrap();
    let maps: Vec<&str> = document
        .split(|x: char| !x.is_numeric() && x != ' ' && x != '\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();
    let almanac = Almanac::parse_almanac(maps);

    println!("{}", find_closest(almanac));
}
