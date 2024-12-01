use std::collections::HashMap;
use std::fs;

struct Beam {
    x: usize,
    y: usize,
    direction: i32,
    vertical: bool,
}

fn spread_horizontal_beam(
    beam: &Beam,
    layout: &Vec<Vec<char>>,
    squares: &mut Vec<Vec<char>>,
    beams: &mut Vec<Beam>,
) {
    let line_count = layout.len();
    let line_len = layout[0].len();
    let Beam {
        mut x,
        y,
        direction,
        ..
    } = *beam;

    let range: Vec<usize>;

    if direction == -1 {
        range = (0..=x).rev().collect();
        x = 0;
    } else {
        range = (x..line_len).collect();
        x = line_len - 1;
    }

    for i in range {
        squares[y][i] = '#';
        if !(layout[y][i] == '.' || layout[y][i] == '#' || layout[y][i] == '-') {
            x = i;
            break;
        }
    }

    let reflects_up = y > 0
        && ((layout[y][x] == '/' && direction == 1)
            || (layout[y][x] == '\\' && direction == -1)
            || layout[y][x] == '|');

    let reflects_down = y < line_count - 1
        && ((layout[y][x] == '/' && direction == -1)
            || (layout[y][x] == '\\' && direction == 1)
            || layout[y][x] == '|');

    if reflects_up {
        beams.push(Beam {
            x,
            y: y - 1,
            direction: -1,
            vertical: true,
        });
    }
    if reflects_down {
        beams.push(Beam {
            x,
            y: y + 1,
            direction: 1,
            vertical: true,
        });
    }
}

fn spread_vertical_beam(
    beam: &Beam,
    layout: &Vec<Vec<char>>,
    squares: &mut Vec<Vec<char>>,
    beams: &mut Vec<Beam>,
) {
    let line_count = layout.len();
    let line_len = layout[0].len();
    let Beam {
        x,
        mut y,
        direction,
        ..
    } = *beam;

    let range: Vec<usize>;

    if direction == -1 {
        range = (0..=y).rev().collect();
        y = 0;
    } else {
        range = (y..line_count).collect();
        y = line_count - 1;
    }

    for i in range {
        squares[i][x] = '#';
        if !(layout[i][x] == '.' || layout[i][x] == '#' || layout[i][x] == '|') {
            y = i;
            break;
        }
    }

    let reflects_left = x > 0
        && ((layout[y][x] == '/' && direction == 1)
            || (layout[y][x] == '\\' && direction == -1)
            || layout[y][x] == '-');

    let reflects_right = x < line_len - 1
        && ((layout[y][x] == '/' && direction == -1)
            || (layout[y][x] == '\\' && direction == 1)
            || layout[y][x] == '-');

    if reflects_left {
        beams.push(Beam {
            x: x - 1,
            y,
            direction: -1,
            vertical: false,
        });
    }
    if reflects_right {
        beams.push(Beam {
            x: x + 1,
            y,
            direction: 1,
            vertical: false,
        });
    }
}

fn hash(beam: &Beam) -> i64 {
    let Beam {
        x, y, direction, ..
    } = *beam;

    (((x + y) * (x + y + 1) / 2 + x) as i64) * direction as i64
}

fn energize_squares(layout: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map: HashMap<i64, Beam> = HashMap::new();
    let mut energized_squares = layout.clone();
    let mut beams: Vec<Beam> = Vec::new();
    beams.push(Beam {
        x: 0,
        y: 0,
        direction: 1,
        vertical: false,
    });

    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let hash = hash(&beam);
        let saved_beam = map.get(&hash);
        if saved_beam.is_none() {
            if beam.vertical {
                spread_vertical_beam(&beam, &layout, &mut energized_squares, &mut beams);
            } else {
                spread_horizontal_beam(&beam, &layout, &mut energized_squares, &mut beams);
            }
            map.insert(hash, beam);
        }
    }
    for line in &energized_squares {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    energized_squares
}

fn main() {
    let document = fs::read_to_string("test.in").unwrap();
    let layout: Vec<Vec<char>> = document.lines().map(|x| x.chars().collect()).collect();

    let mut sum = 0;
    for line in energize_squares(layout) {
        sum += line.iter().filter(|&x| *x == '#').count();
    }

    println!("{}", sum);
}
