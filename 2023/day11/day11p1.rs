use std::cmp::{max, min};
use std::fs;

fn get_empty_count(start: usize, end: usize, vec: &Vec<bool>) -> usize {
    let mut count = 0;
    for i in start..end {
        if vec[i] {
            count += 1;
        }
    }
    count
}

fn get_distance_sum(map: Vec<Vec<char>>) -> usize {
    let mut sum = 0;
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

    for i in 0..row_count {
        for j in 0..column_count {
            if map[i][j] == '#' {
                for k in i..row_count {
                    for l in 0..column_count {
                        if (l > j || k > i) && map[k][l] == '#' {
                            let ord_j = min(j, l);
                            let ord_l = max(j, l);
                            let d_y = k - i;
                            let d_x = ord_l - ord_j;
                            let empty_rows = get_empty_count(i, k, &rows);
                            let empty_columns = get_empty_count(ord_j, ord_l, &columns);
                            sum += d_y + empty_columns + d_x + empty_rows;
                        }
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let document = fs::read_to_string("day11.in").unwrap();
    let map = document
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("{}", get_distance_sum(map));
}
