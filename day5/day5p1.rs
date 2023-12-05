use std::fs;

struct Almanac {
    seeds: Vec<u32>,
    maps: [Vec<Vec<u32>>; 7],
}

impl Almanac {
    fn parse_almanac(lines: Vec<&str>) -> Self {
        let seeds = parse_line(lines[0]);
        let mut maps: [Vec<Vec<u32>>; 7] = Default::default();
        for i in 0..7 {
            maps[i] = parse_map(lines[i + 1]);
        }
        Self { seeds, maps }
    }
}

fn parse_line(line: &str) -> Vec<u32> {
    let line = line.split_whitespace();
    let mut nums: Vec<u32> = Vec::new();

    for num in line {
        nums.push(num.parse().unwrap());
    }

    nums
}

fn parse_map(nums: &str) -> Vec<Vec<u32>> {
    let nums = nums.lines();
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in nums {
        map.push(parse_line(line));
    }

    map
}

fn main() {
    let document = fs::read_to_string("test.in").unwrap();
    let maps: Vec<&str> = document
        .split(|x: char| !x.is_numeric() && x != ' ' && x != '\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();
    let almanac = Almanac::parse_almanac(maps);
}
