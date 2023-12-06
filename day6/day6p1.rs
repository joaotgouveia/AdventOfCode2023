use std::fs;

struct Race {
    t: u32,
    d: u32,
}
impl Race {
    fn parse(time: &str, distance: &str) -> Self {
        Self {
            t: time.parse().unwrap(),
            d: distance.parse().unwrap(),
        }
    }

    fn get_record_count(self) -> u32 {
        let mut records = 0;
        for t in 1..self.t {
            if (self.t - t) * t > self.d {
                records += 1;
            }
        }
        records
    }
}
fn main() {
    let document = fs::read_to_string("day6.in").unwrap();
    let mut lines = document.lines();

    let time = &lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1..];
    let distance = &lines
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[1..];

    let mut races: Vec<Race> = Vec::new();
    for i in 0..time.len() {
        races.push(Race::parse(time[i], distance[i]));
    }

    let mut record_count = 1;
    for race in races {
        record_count *= race.get_record_count();
    }

    println!("{}", record_count);
}
