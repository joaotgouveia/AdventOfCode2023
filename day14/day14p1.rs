use std::fs;

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

fn main() {
    let document = fs::read_to_string("day14.in").unwrap();
    let dish: Vec<Vec<char>> = document.lines().map(|x| x.chars()).collect();
    
    let line_len = maps[0].len();
    let line_count = maps.len();
    
    for i in 0..line_count {
        for j in 0..line_len {
            move_north(&mut dish, (i, j));
        }
    }

    let mut load = 0;
    for i in 0..line_count {
        load += dish[i].iter().filter(|x| x == 'O').count() * (line_count - i);
    }

    println!("{}", load);
}
