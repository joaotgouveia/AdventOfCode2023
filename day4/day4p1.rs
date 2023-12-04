use std::fs;

fn parse_card(card: &str) -> (Vec<u32>, Vec<u32>) {
    let mut card = card.split(":");
    card.next();
    let card: Vec<&str> = card.next().unwrap().split("|").collect();

    (
        card[0]
            .split(" ")
            .filter(|x| x.parse::<u32>().is_ok())
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>(),
        card[1]
            .split(" ")
            .filter(|x| x.parse::<u32>().is_ok())
            .map(|x| x.parse().unwrap())
            .collect::<Vec<u32>>(),
    )
}

fn get_points(card: &str) -> u32 {
    let (card, winning_cards) = parse_card(card);
    let mut win_count = 0;
    for num in card {
        if winning_cards.contains(&num) {
            win_count += 1
        }
    }
    if win_count != 0 {
        2_u32.pow(win_count - 1)
    } else {
        0
    }
}

fn main() {
    let document = fs::read_to_string("day4.in").unwrap();
    let mut sum = 0;

    for card in document.lines() {
        sum += get_points(card);
    }

    println!("{}", sum);
}
