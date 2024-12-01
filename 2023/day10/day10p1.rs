use std::convert::TryInto;
use std::fs;

struct Point {
    x: usize,
    y: usize,
}

fn find_starting_pipes(start: &Point, map: &Vec<Vec<char>>) -> Vec<Point> {
    let mut pipes: Vec<Point> = Vec::new();

    if map[start.y][start.x - 1] != '.' {
        match map[start.y][start.x - 1] {
            '-' | 'L' | 'F' => pipes.push(Point {
                x: start.x - 1,
                y: start.y,
            }),
            _ => (),
        }
    }

    if map[start.y][start.x + 1] != '.' {
        match map[start.y][start.x - 1] {
            '-' | 'J' | '7' => pipes.push(Point {
                x: start.x + 1,
                y: start.y,
            }),
            _ => (),
        }
    }

    if map[start.y - 1][start.x] != '.' {
        match map[start.y][start.x - 1] {
            '|' | 'L' | 'J' => pipes.push(Point {
                x: start.x,
                y: start.y - 1,
            }),
            _ => (),
        }
    }

    if map[start.y + 1][start.x] != '.' {
        match map[start.y][start.x - 1] {
            '|' | 'F' | '7' => pipes.push(Point {
                x: start.x,
                y: start.y + 1,
            }),
            _ => (),
        }
    }

    pipes
}

fn move_l(old_pipe: &Point, pipe: &Point) -> Point {
    let next_pipe = Point {
        x: pipe.x,
        y: pipe.y - 1,
    };

    if next_pipe.y != old_pipe.y || next_pipe.x != old_pipe.x {
        return next_pipe;
    }

    Point {
        x: pipe.x + 1,
        y: pipe.y,
    }
}

fn move_j(old_pipe: &Point, pipe: &Point) -> Point {
    let next_pipe = Point {
        x: pipe.x,
        y: pipe.y - 1,
    };

    if next_pipe.y != old_pipe.y || next_pipe.x != old_pipe.x {
        return next_pipe;
    }

    Point {
        x: pipe.x - 1,
        y: pipe.y,
    }
}

fn move_7(old_pipe: &Point, pipe: &Point) -> Point {
    let next_pipe = Point {
        x: pipe.x,
        y: pipe.y + 1,
    };

    if next_pipe.y != old_pipe.y || next_pipe.x != old_pipe.x {
        return next_pipe;
    }

    Point {
        x: pipe.x - 1,
        y: pipe.y,
    }
}

fn move_f(old_pipe: &Point, pipe: &Point) -> Point {
    let next_pipe = Point {
        x: pipe.x,
        y: pipe.y + 1,
    };

    if next_pipe.y != old_pipe.y || next_pipe.x != old_pipe.x {
        return next_pipe;
    }

    Point {
        x: pipe.x + 1,
        y: pipe.y,
    }
}

fn move_vertical(old_pipe: &Point, pipe: &Point) -> Point {
    let next_pipe = Point {
        x: pipe.x,
        y: pipe.y + 1,
    };

    if next_pipe.y != old_pipe.y || next_pipe.x != old_pipe.x {
        return next_pipe;
    }

    Point {
        x: pipe.x,
        y: pipe.y - 1,
    }
}

fn move_horizontal(old_pipe: &Point, pipe: &Point) -> Point {
    let next_pipe = Point {
        x: pipe.x + 1,
        y: pipe.y,
    };

    if next_pipe.y != old_pipe.y || next_pipe.x != old_pipe.x {
        return next_pipe;
    }

    Point {
        x: pipe.x - 1,
        y: pipe.y,
    }
}

fn next_pipe(old_pipe: &mut Point, pipe: &mut Point, map: &Vec<Vec<char>>) {
    let aux = Point {
        x: pipe.x,
        y: pipe.y,
    };

    match map[pipe.y][pipe.x] {
        'L' => *pipe = move_l(old_pipe, pipe),
        'J' => *pipe = move_j(old_pipe, pipe),
        'F' => *pipe = move_f(old_pipe, pipe),
        '7' => *pipe = move_7(old_pipe, pipe),
        '|' => *pipe = move_vertical(old_pipe, pipe),
        '-' => *pipe = move_horizontal(old_pipe, pipe),
        _ => panic!("Unrecognized pipe"),
    }

    *old_pipe = aux;
}

fn get_farthest_point(start: Point, map: Vec<Vec<char>>) -> u32 {
    let mut steps = 1;
    let mut pipes = find_starting_pipes(&start, &map);
    let mut pipe_2 = pipes.pop().unwrap();
    let mut pipe_1 = pipes.pop().unwrap();
    let mut old_pipe_2 = Point {
        x: start.x,
        y: start.y,
    };
    let mut old_pipe_1 = start;

    while pipe_1.x != pipe_2.x || pipe_1.y != pipe_2.y {
        steps += 1;
        next_pipe(&mut old_pipe_1, &mut pipe_1, &map);
        next_pipe(&mut old_pipe_2, &mut pipe_2, &map);
    }

    steps
}

fn find_animal(map: &Vec<Vec<char>>) -> Point {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
            }
        }
    }
    panic!("Couldn't find the animal");
}

fn main() {
    let document = fs::read_to_string("day10.in").unwrap();
    let map = document
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = find_animal(&map);

    println!("{}", get_farthest_point(start, map));
}
