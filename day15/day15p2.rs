use std::fs;

fn box_insert<'a>(hash_map: &mut Vec<Vec<(&'a str, usize)>>, step: &'a str) {
    let mut info = step
        .split(|x| x == '=' || x == '-')
        .filter(|x| !x.is_empty());
    let label = info.next().unwrap();
    let focal_length = info.next();
    let hashed_label = hash(label);

    for i in 0..hash_map[hashed_label].len() {
        if hash_map[hashed_label][i].0.eq(label) {
            if focal_length.is_some() {
                hash_map[hashed_label][i].1 = focal_length.unwrap().parse().unwrap();
            } else {
                hash_map[hashed_label].remove(i);
            }
            return;
        }
    }
    if focal_length.is_some() {
        hash_map[hashed_label].push((label, focal_length.unwrap().parse().unwrap()));
    }
}

fn hash(string: &str) -> usize {
    let mut hashed_string = 0;
    for c in string.chars() {
        hashed_string += c as usize;
        hashed_string *= 17;
        hashed_string %= 256;
    }

    hashed_string
}

fn main() {
    let document = fs::read_to_string("day15.in").unwrap();
    let initialization_sequence = document.split(",").collect::<Vec<&str>>();
    let mut hash_map: Vec<Vec<(&str, usize)>> = Vec::new();

    for _ in 0..256 {
        hash_map.push(Vec::new());
    }

    for step in initialization_sequence {
        box_insert(&mut hash_map, step);
    }

    let mut sum = 0;
    for i in 0..256 {
        for j in 0..hash_map[i].len() {
            sum += (i + 1) * (j + 1) * hash_map[i][j].1;
        }
    }

    println!("{}", sum);
}
