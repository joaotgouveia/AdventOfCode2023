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

fn check_test_area(x: f64, y: f64) -> bool {
    const TEST_AREA_MIN: f64 = 200000000000000.0;
    const TEST_AREA_MAX: f64 = 400000000000000.0;

    x >= TEST_AREA_MIN && y >= TEST_AREA_MIN && x <= TEST_AREA_MAX && y <= TEST_AREA_MAX
}

fn same_sign(i: f64, j: f64) -> bool {
    (i < 0.0 && j < 0.0) || (i > 0.0 && j > 0.0)
}

fn intersects(l1: &Line, l2: &Line) -> bool {
    let a1 = (l1.y - (l1.y + l1.v_y)) / (l1.x - (l1.x + l1.v_x));
    let a2 = (l2.y - (l2.y + l2.v_y)) / (l2.x - (l2.x + l2.v_x));

    let b1 = l1.y - a1 * l1.x;
    let b2 = l2.y - a2 * l2.x;

    if a1 != a2 {
        let x = (b2 - b1) / (a1 - a2);
        let y = x * a1 + b1;

        if same_sign(x - l1.x, l1.v_x) && same_sign(x - l2.x, l2.v_x) {
            return check_test_area(x, y);
        }
    }

    false
}

fn main() {
    let document = fs::read_to_string("day24.in").unwrap();
    let mut lines: Vec<Line> = Vec::new();

    for data in document.lines() {
        lines.push(Line::parse(data));
    }

    let len = lines.len();
    let mut sum = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            if intersects(&lines[i], &lines[j]) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
