use std::collections::HashMap;
use std::fs;

fn move_east(dish: &mut Vec<Vec<char>>, point: (usize, usize)) {
    if dish[point.0][point.1] != 'O' {
        return;
    }

    let max = dish[0].len() - 1;
    let mut new_point = point;
    while new_point.1 < max && dish[new_point.0][new_point.1 + 1] == '.' {
        new_point.1 += 1;
    }

    dish[point.0][point.1] = '.';
    dish[new_point.0][new_point.1] = 'O';
}

fn move_west(dish: &mut Vec<Vec<char>>, point: (usize, usize)) {
    if dish[point.0][point.1] != 'O' {
        return;
    }

    let mut new_point = point;
    while new_point.1 > 0 && dish[new_point.0][new_point.1 - 1] == '.' {
        new_point.1 -= 1;
    }

    dish[point.0][point.1] = '.';
    dish[new_point.0][new_point.1] = 'O';
}

fn move_south(dish: &mut Vec<Vec<char>>, point: (usize, usize)) {
    if dish[point.0][point.1] != 'O' {
        return;
    }

    let max = dish.len() - 1;
    let mut new_point = point;
    while new_point.0 < max && dish[new_point.0 + 1][new_point.1] == '.' {
        new_point.0 += 1;
    }

    dish[point.0][point.1] = '.';
    dish[new_point.0][new_point.1] = 'O';
}

fn move_north(dish: &mut Vec<Vec<char>>, point: (usize, usize)) {
    if dish[point.0][point.1] != 'O' {
        return;
    }

    let mut new_point = point;
    while new_point.0 > 0 && dish[new_point.0 - 1][new_point.1] == '.' {
        new_point.0 -= 1;
    }

    dish[point.0][point.1] = '.';
    dish[new_point.0][new_point.1] = 'O';
}

fn cycle(dish: &mut Vec<Vec<char>>) {
    let line_len = dish[0].len();
    let line_count = dish.len();

    for i in 0..line_count {
        for j in 0..line_len {
            move_north(dish, (i, j));
        }
    }

    for i in 0..line_count {
        for j in 0..line_len {
            move_west(dish, (i, j));
        }
    }

    for i in (0..line_count).rev() {
        for j in 0..line_len {
            move_south(dish, (i, j));
        }
    }

    for i in 0..line_count {
        for j in (0..line_len).rev() {
            move_east(dish, (i, j));
        }
    }
}

fn hash(dish: &Vec<Vec<char>>) -> usize {
    let mut hash = 0;
    let line_count = dish.len();
    let line_len = dish[0].len();

    for i in 0..line_count {
        for j in 0..line_len {
            if dish[i][j] == 'O' {
                hash += j;
            }
        }
        hash *= i * 13;
        hash %= 9973;
    }

    hash
}

fn get_load(dish: &Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let line_count = dish.len();
    for i in 0..line_count {
        load += dish[i].iter().filter(|&x| *x == 'O').count() * (line_count - i);
    }
    load
}

fn main() {
    const CYCLES: usize = 1000000000;
    let document = fs::read_to_string("day14.in").unwrap();
    let mut dish: Vec<Vec<char>> = document.lines().map(|x| x.chars().collect()).collect();
    let mut map: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
    let mut hashes: Vec<usize> = Vec::new();

    let mut cycle_start = 0;
    for _ in 0..CYCLES {
        let hash = hash(&dish);
        if map.contains_key(&hash) {
            cycle_start = hash;
            break;
        }
        map.insert(hash, dish.clone());
        hashes.push(hash);
        cycle(&mut dish);
    }

    let start_index = hashes.iter().position(|&x| x == cycle_start).unwrap();
    let final_index = (CYCLES - start_index) % (hashes.len() - start_index) + start_index;
    let final_dish = map.get(&hashes[final_index]).unwrap();
    println!("{}", get_load(final_dish));
}
