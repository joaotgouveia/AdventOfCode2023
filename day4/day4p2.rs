use std::convert::TryFrom;
use std::fs;

struct Card {
    id: usize,
    nums: Vec<usize>,
    winning_nums: Vec<usize>,
}

impl Card {
    fn parse_card(card: &str) -> Self {
        let mut card = card.split(":");

        let id: usize = card
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap()
            - 1;
        let card: Vec<&str> = card.next().unwrap().split("|").collect();

        Self {
            id,
            nums: card[0]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>(),
            winning_nums: card[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>(),
        }
    }
}

fn get_card_count(cards: &Vec<Card>, card: &Card, table: &mut Vec<i32>) -> usize {
    if table[card.id] != -1 {
        return usize::try_from(table[card.id]).unwrap();
    }

    let mut count: usize = 0;
    for num in &card.nums {
        if card.winning_nums.contains(&num) {
            count += 1;
        }
    }

    for i in 0..count {
        count += get_card_count(&cards, &cards[card.id + i + 1], table);
    }

    table[card.id] = i32::try_from(count).unwrap();

    count
}

fn main() {
    let document = fs::read_to_string("day4.in").unwrap();
    let card_count = document.lines().count();
    let mut table: Vec<i32> = vec![-1; card_count];
    let mut cards: Vec<Card> = Vec::new();
    let mut sum: usize = card_count;

    for card in document.lines() {
        cards.push(Card::parse_card(card));
    }

    for card in &cards {
        sum += get_card_count(&cards, &card, &mut table);
    }

    println!("{}", sum);
}
