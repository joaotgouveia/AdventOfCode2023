use std::fs;

struct Game {
    id: u32,
    hints: Vec<Hint>,
}

struct Hint {
    green: u32,
    blue: u32,
    red: u32,
}

impl Hint {
    fn new() -> Self {
        Self {
            green: 0,
            blue: 0,
            red: 0,
        }
    }
}

fn parse_hint(hint_str: &str) -> Hint {
    let hint_str: Vec<&str> = hint_str.split(", ").collect();
    let mut hint = Hint::new();

    for set_str in hint_str {
        let set: Vec<&str> = set_str.split(" ").collect();
        match set[1] {
            "green" => hint.green = set[0].parse().unwrap(),
            "blue" => hint.blue = set[0].parse().unwrap(),
            "red" => hint.red = set[0].parse().unwrap(),
            _ => panic!("Unexpected color"),
        };
    }

    hint
}

fn parse_game(game: &str) -> Game {
    // Game X: Hint1; Hint2; ...
    let game = &game[5..];
    let params: Vec<&str> = game.split(": ").collect();

    let id: u32 = params[0].parse().unwrap();
    let hints_str: Vec<&str> = params[1].split("; ").collect();

    let mut hints: Vec<Hint> = Vec::new();
    for hint_str in hints_str {
        hints.push(parse_hint(hint_str));
    }

    Game { id, hints }
}

fn get_id(game: &str, target: &Hint) -> u32 {
    let game = parse_game(game);
    for hint in game.hints {
        if hint.green > target.green || hint.blue > target.blue || hint.red > target.red {
            return 0;
        }
    }

    game.id
}

fn main() {
    let mut sum = 0;
    let document = fs::read_to_string("day2.in").unwrap();

    let target = Hint {
        green: 13,
        blue: 14,
        red: 12,
    };

    for game in document.lines() {
        sum += get_id(game, &target);
    }

    println!("{}", sum);
}
