use std::fs;

fn get_distance_sum(map: Vec<Vec<char>>) -> i64 {
    let row_count = map.len();
    let column_count = map[0].len();
    let mut sum = 0;
    for i in 0..row_count {
        for j in 0..column_count {
            if map[i][j] == '#' {
                for k in i..row_count {
                    for l in 0..column_count {
                        if (l > j || k > i) && map[k][l] == '#' {
                            let d_y: i64 = k as i64 - i as i64;
                            let d_x: i64 = l as i64 - j as i64;
                            sum += d_y.abs() + d_x.abs();
                        }
                    }
                }
            }
        }
    }
    sum
}

fn expand_galaxies(map: &mut Vec<Vec<char>>) {
    let row_count = map.len();
    let column_count = map[0].len();
    let mut rows = vec![true; row_count];
    let mut columns = vec![true; column_count];

    for i in 0..row_count {
        for j in 0..column_count {
            if map[i][j] == '#' {
                rows[i] = false;
                columns[j] = false;
            }
        }
    }

    let mut i = 0;
    let mut added_rows = 0;
    while i < row_count + added_rows {
        if rows[i - added_rows] {
            let new_row = map[i].clone();
            map.insert(i, new_row);
            added_rows += 1;
            i += 1;
        }
        i += 1;
    }

    let mut j = 0;
    let mut added_columns = 0;
    while j < column_count + added_columns {
        if columns[j - added_columns] {
            for i in 0..(row_count + added_rows) {
                map[i].insert(j, '.');
            }
            added_columns += 1;
            j += 1;
        }
        j += 1;
    }
}

fn main() {
    let document = fs::read_to_string("day11.in").unwrap();
    let mut map = document
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    expand_galaxies(&mut map);

    println!("{}", get_distance_sum(map));
}
