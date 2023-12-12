use std::fs;

fn main() {
    let document = fs::read_to_string("test.in").unwrap();
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

        sum += 0;
    }

    println!("{}", sum);
}
