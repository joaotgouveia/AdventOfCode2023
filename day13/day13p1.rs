use std::fs;

fn reflects(
    map: &Vec<&str>,
    line: usize,
    n: usize,
    eq: &dyn Fn(usize, usize, &Vec<&str>) -> bool,
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

fn summarize_map(map: &str) -> usize {
    let map = map.lines().collect::<Vec<&str>>();
    let rows = map.len();
    let cols = map[0].len();

    fn column_eq(i: usize, j: usize, map: &Vec<&str>) -> bool {
        let n = map.len();
        for k in 0..n {
            if map[k].chars().nth(i).unwrap() != map[k].chars().nth(j).unwrap() {
                return false;
            }
        }

        true
    }

    fn row_eq(i: usize, j: usize, map: &Vec<&str>) -> bool {
        map[i].eq(map[j])
    }

    for i in 1..rows {
        if reflects(&map, i, rows, &row_eq) {
            return i * 100;
        }
    }

    for i in 1..cols {
        if reflects(&map, i, cols, &column_eq) {
            return i;
        }
    }

    panic!("Couldn't find reflexion");
}

fn main() {
    let document = fs::read_to_string("day13.in").unwrap();
    let maps = document.split("\n\n").collect::<Vec<&str>>();
    let mut sum = 0;

    for map in maps {
        sum += summarize_map(map);
    }

    println!("{}", sum);
}
