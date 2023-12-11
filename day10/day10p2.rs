use std::fs;

fn fill(i: usize, j: usize, map: &mut Vec<Vec<char>>, color: char) {
    if map[i][j] != '.' {
        return;
    }

    let row_count = map.len();
    let column_count = map[0].len();

    map[i][j] = color;

    if i > 0 {
        fill(i - 1, j, map, color);
    }
    if i < row_count - 1 {
        fill(i + 1, j, map, color);
    }
    if j > 0 {
        fill(i, j - 1, map, color);
    }
    if j < column_count - 1 {
        fill(i, j + 1, map, color);
    }
}

fn flood_fill(map: &mut Vec<Vec<char>>) {
    let row_count = map.len();
    let column_count = map[0].len();
    let mut color = 'a';

    for i in 0..row_count {
        for j in 0..column_count {
            if map[i][j] == '.' {
                fill(i, j, map, color);
                color = (color as u8 + 1) as char;
            }
        }
    }
}

fn print_map(map: Vec<Vec<char>>) {
    for line in map {
        for point in line {
            print!("{}", point);
        }
        println!("");
    }
}

fn main() {
    let document = fs::read_to_string("test.in").unwrap();
    let mut map = document
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    flood_fill(&mut map);

    print_map(map);

    println!("{}", 0);
}
