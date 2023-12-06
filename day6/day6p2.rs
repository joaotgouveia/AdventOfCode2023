use std::fs;

struct Race {
    t: u64,
    d: u64,
}
impl Race {
    fn parse(time: &str, distance: &str) -> Self {
        Self {
            t: time.parse().unwrap(),
            d: distance.parse().unwrap(),
        }
    }

    fn get_record_count(self) -> u64 {
        let mut records: u64 = 0;
        for t in 1..self.t {
            if (self.t - t) * t > self.d {
                records += 1;
            } else if records > 0 {
                break;
            }
        }
        records
    }
}
fn main() {
    let document = fs::read_to_string("day6.in").unwrap();
    let mut lines = document.lines();

    let mut time = lines.next().unwrap().split_whitespace();
    time.next();
    let time = time.collect::<String>();

    let mut distance = lines.next().unwrap().split_whitespace();
    distance.next();
    let distance = distance.collect::<String>();

    let race = Race::parse(&time, &distance);

    println!("{}", race.get_record_count());
}
