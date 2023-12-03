use std::fs;

struct Set {
    green: u32,
    blue: u32,
    red: u32,
}

impl Set {
    fn new() -> Self {
        Self {
            green: 0,
            blue: 0,
            red: 0,
        }
    }

    fn parse_set(set_str: &str) -> Self {
        let set_str: Vec<&str> = set_str.split(", ").collect();
        let mut set = Self::new();

        for cubes_str in set_str {
            let cubes: Vec<&str> = cubes_str.split(" ").collect();
            match cubes[1] {
                "green" => set.green = cubes[0].parse().unwrap(),
                "blue" => set.blue = cubes[0].parse().unwrap(),
                "red" => set.red = cubes[0].parse().unwrap(),
                _ => panic!("Unexpected color"),
            };
        }
        set
    }

    fn power(&self) -> u32 {
        self.green * self.blue * self.red
    }
}

struct Game {
    id: u32,
    hints: Vec<Set>,
}

impl Game {
    fn parse_game(game: &str) -> Self {
        // Game X: Hint1; Hint2; ...
        let game = &game[5..];
        let params: Vec<&str> = game.split(": ").collect();

        let id: u32 = params[0].parse().unwrap();
        let hints_str: Vec<&str> = params[1].split("; ").collect();

        let mut hints: Vec<Set> = Vec::new();
        for hint_str in hints_str {
            hints.push(Set::parse_set(hint_str));
        }

        Self { id, hints }
    }
}

fn get_power(game: &str) -> u32 {
    let game = Game::parse_game(game);
    let mut fewes_cubes = Set::new();
    for hint in game.hints {
        if hint.green > fewes_cubes.green {
            fewes_cubes.green = hint.green;
        }
        if hint.blue > fewes_cubes.blue {
            fewes_cubes.blue = hint.blue;
        }
        if hint.red > fewes_cubes.red {
            fewes_cubes.red = hint.red;
        }
    }

    fewes_cubes.power()
}

fn main() {
    let mut sum = 0;
    let document = fs::read_to_string("day2.in").unwrap();

    for game in document.lines() {
        sum += get_power(game);
    }

    println!("{}", sum);
}
