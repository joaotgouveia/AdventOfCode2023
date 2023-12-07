use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fs;

struct Hand {
    strength: u32,
    cards: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let mut line = line.split_whitespace();
        let hand_str = line.next().unwrap();
        let bid: u32 = line.next().unwrap().parse().unwrap();

        let mut cards: Vec<u32> = Vec::new();
        let mut counts: [u32; 13] = [0; 13];
        let mut jokers = 0;

        for char in hand_str.chars() {
            let card: u32 = match char {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                'J' => 1,
                num => num.to_digit(10).unwrap(),
            };
            if card == 1 {
                jokers += 1;
            } else {
                counts[(card - 2) as usize] += 1;
            }
            cards.push(card);
        }

        let mut strength: u32 = 0;
        let mut pair_count: u32 = 0;
        let mut has_three = false;

        for count in counts {
            if count == 5 || count == 4 {
                strength = count + 1;
            } else if count == 3 {
                has_three = true;
            } else if count == 2 {
                pair_count += 1;
            }
            if strength != 0 || (has_three && pair_count == 1) || pair_count == 2 {
                break;
            }
        }

        if strength == 0 {
            if has_three {
                strength = 3 + pair_count;
            } else {
                strength = pair_count;
            }
        }

        if jokers == 5 || jokers == 4 {
            strength = 6;
        } else if jokers == 3 {
            strength = 5 + pair_count;
        } else if jokers == 2 {
            if has_three {
                strength = 6;
            } else {
                strength = 3 + 2 * pair_count;
            }
        } else if jokers == 1 {
            if strength == 5 {
                strength = 6;
            } else if strength == 0 {
                strength = 1;
            } else {
                strength += 2;
            }
        }

        Self {
            strength,
            cards,
            bid,
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.strength == other.strength {
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength != other.strength {
            return self.strength.cmp(&other.strength);
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }
        Ordering::Equal
    }
}

fn main() {
    let document = fs::read_to_string("day7.in").unwrap();
    let mut hands: Vec<Hand> = Vec::new();
    for line in document.lines() {
        hands.push(Hand::parse(line));
    }

    hands.sort_unstable();
    let mut sum = 0;
    for i in 0..hands.len() {
        sum += hands[i].bid * ((i + 1) as u32);
    }
    println!("{}", sum);
}
