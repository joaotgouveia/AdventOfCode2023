use std::fs;

struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn parse(string: &str) -> Self {
        let mut coordinates = string.split(",").map(|x| x.parse::<usize>().unwrap());
        Self {
            x: coordinates.next().unwrap(),
            y: coordinates.next().unwrap(),
            z: coordinates.next().unwrap(),
        }
    }
}

struct Brick {
    id: usize,
    start: Point,
    end: Point,
    supports: Vec<usize>,
    supported: usize,
}

impl Brick {
    fn parse(id: usize, string: &str) -> Self {
        let mut points = string.split("~").map(|x| Point::parse(x));
        Self {
            id,
            start: points.next().unwrap(),
            end: points.next().unwrap(),
            supports: Vec::new(),
            supported: 0,
        }
    }
}

fn overlaps(brick1: &Brick, brick2: &Brick) -> bool {
    let overlaps_x;
    let overlaps_y;

    if brick1.start.x < brick2.start.x {
        overlaps_x = brick1.end.x >= brick2.start.x;
    } else {
        overlaps_x = brick2.end.x >= brick1.start.x;
    }

    if brick1.start.y < brick2.start.y {
        overlaps_y = brick1.end.y >= brick2.start.y;
    } else {
        overlaps_y = brick2.end.y >= brick1.start.y;
    }

    overlaps_x && overlaps_y
}

fn stable(bricks: &Vec<Brick>, start: usize) -> bool {
    for i in start..bricks.len() {
        let mut supported = false;
        for j in 0..i {
            if supports(&bricks[j], &bricks[i]) {
                supported = true;
            }
        }
        if !supported {
            return false;
        }
    }

    true
}

fn supports(brick1: &Brick, brick2: &Brick) -> bool {
    brick1.end.z == brick2.start.z - 1 && overlaps(&brick1, &brick2)
}

fn supported(i: usize, bricks: &Vec<Brick>) -> bool {
    for j in 0..i {
        if supports(&bricks[j], &bricks[i]) {
            return true;
        }
    }

    false
}

fn get_supporters(i: usize, bricks: &mut Vec<Brick>) {
    for j in 0..i {
        if supports(&bricks[j], &bricks[i]) {
            let id = bricks[i].id;
            bricks[j].supports.push(id);
            bricks[i].supported += 1;
        }
    }
}

fn let_bricks_fall(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        if bricks[i].start.z > 1 {
            while !supported(i, bricks) && bricks[i].start.z > 1 {
                bricks[i].start.z -= 1;
                bricks[i].end.z -= 1;
            }
        }
    }
}

fn main() {
    let document = fs::read_to_string("day22.in").unwrap();
    let mut bricks: Vec<Brick> = Vec::new();

    for (i, line) in document.lines().enumerate() {
        bricks.push(Brick::parse(i, line));
    }

    bricks.sort_unstable_by_key(|x| x.start.z);
    let_bricks_fall(&mut bricks);
    bricks.sort_unstable_by_key(|x| x.start.z);
    for i in 0..bricks.len() {
        get_supporters(i, &mut bricks);
    }
    bricks.sort_unstable_by_key(|x| x.id);

    let mut sum = 0;
    for i in 0..bricks.len() {
        if bricks[i].supports.is_empty() {
            sum += 1;
        } else {
            for id in &bricks[i].supports {
                if bricks[*id].supported > 1 {
                    sum += 1;
                    break;
                }
            }
        }
    }

    println!("{}", sum);
}
