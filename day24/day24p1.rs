use std::fs;

struct Line {
    x: f64,
    y: f64,
    v_x: f64,
    v_y: f64,
}

impl Line {
    fn parse(string: &str) -> Self {
        let mut data = string.split(" @ ");
        let mut position = data.next().unwrap().split(", ").map(|x| x.trim());
        let x: f64 = position.next().unwrap().parse().unwrap();
        let y: f64 = position.next().unwrap().parse().unwrap();
        let mut velocity = data.next().unwrap().split(", ").map(|x| x.trim());
        let v_x: f64 = velocity.next().unwrap().parse().unwrap();
        let v_y: f64 = velocity.next().unwrap().parse().unwrap();

        Self { x, y, v_x, v_y }
    }
}

fn intersects(l1: &Line, l2: &Line) -> u64 {
    let t_x = (l1.x - l2.x) / (l2.v_x - l1.v_x);
    let t_y = (l1.y - l2.y) / (l2.v_y - l1.v_y);

    if t_x == t_y && t_x <= 400000000000000.0 && t_x >= 200000000000000.0 {
        return 1;
    }

    0
}

fn main() {
    let document = fs::read_to_string("test.in").unwrap();
    let mut lines: Vec<Line> = Vec::new();

    for data in document.lines() {
        lines.push(Line::parse(data));
    }

    let len = lines.len();
    let mut sum = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            sum += intersects(&lines[i], &lines[j]);
        }
    }

    println!("{}", sum);
}
