use std::fs;

fn reflects(
    map: &Vec<Vec<char>>,
    line: usize,
    n: usize,
    eq: &dyn Fn(usize, usize, &Vec<Vec<char>>) -> bool,
) -> bool {
    let mut left = (line - 1) as i64;
    let mut right = line;

    while left > -1 && right < n {
        if !eq(left as usize, right, map) {
            return false;
        }
        left -= 1;
        right += 1;
    }

    true
}

fn summarize_map(map: &Vec<Vec<char>>) -> Vec<usize> {
    let rows = map.len();
    let cols = map[0].len();
    let mut reflection_lines: Vec<usize> = Vec::new();

    fn column_eq(i: usize, j: usize, map: &Vec<Vec<char>>) -> bool {
        let n = map.len();
        for k in 0..n {
            if map[k][i] != map[k][j] {
                return false;
            }
        }

        true
    }

    fn row_eq(i: usize, j: usize, map: &Vec<Vec<char>>) -> bool {
        let n = map[0].len();
        for k in 0..n {
            if map[i][k] != map[j][k] {
                return false;
            }
        }

        true
    }

    for i in 1..rows {
        if reflects(&map, i, rows, &row_eq) {
            reflection_lines.push(i * 100);
        }
    }

    for i in 1..cols {
        if reflects(&map, i, cols, &column_eq) {
            reflection_lines.push(i);
        }
    }

    reflection_lines
}

fn summarize_fixed_map(map: &str) -> usize {
    let map = map.lines().collect::<Vec<&str>>();
    let rows = map.len();
    let cols = map[0].len();

    let mut fixed_map: Vec<Vec<char>> = Vec::new();
    for line in map {
        fixed_map.push(line.chars().collect::<Vec<char>>());
    }

    let original_reflection = summarize_map(&fixed_map)[0];

    for i in 0..rows {
        for j in 0..cols {
            let smudge = fixed_map[i][j];
            if smudge == '.' {
                fixed_map[i][j] = '#';
            } else {
                fixed_map[i][j] = '.';
            }
            let new_reflections = summarize_map(&fixed_map);
            if new_reflections.len() != 0 {
                if new_reflections.len() == 2 {
                    if new_reflections[0] != original_reflection {
                        return new_reflections[0];
                    }
                    return new_reflections[1];
                }
                if new_reflections[0] != original_reflection {
                    return new_reflections[0];
                }
            }
            fixed_map[i][j] = smudge;
        }
    }

    panic!("No valid smudge");
}

fn main() {
    let document = fs::read_to_string("day13.in").unwrap();
    let maps = document.split("\n\n").collect::<Vec<&str>>();
    let mut sum = 0;

    for map in maps {
        sum += summarize_fixed_map(map);
    }

    println!("{}", sum);
}
