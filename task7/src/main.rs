use core::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(PartialEq, Eq)]
enum JMeaning {
    Jack,
    Joker
}

static mut j_meaning: JMeaning = JMeaning::Jack;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    Pair,
    HighCard
}

impl HandType {
    fn get(cards: &Vec<char>) -> Self {
        let mut checked_values = HashSet::new();
        let mut pairs = 0;
        let mut three_of_a_kinds = 0;
        for card in cards {
            if checked_values.contains(card) {
                continue;
            }
            checked_values.insert(card);
            match cards.iter().filter(|&n| *n == *card).count() {
                2 => pairs += 1,
                3 => three_of_a_kinds +=1,
                4 => return HandType::FourOfAKind,
                5 => return HandType::FiveOfAKind,
                6 => return HandType::FiveOfAKind,
                _ => continue
            };
        }
        if three_of_a_kinds == 1 && pairs == 1 {
            HandType::FullHouse
        } else if three_of_a_kinds == 1 {
            HandType::ThreeOfAKind
        } else if pairs == 2 {
            HandType::TwoPairs
        } else if pairs == 1 {
            HandType::Pair
        } else {
            HandType:: HighCard
        }
    }

    fn repetitions_to_type(repetitions: u32) -> Self {
        match repetitions {
            1 => HandType::HighCard,
            2 => HandType::Pair,
            3 => HandType::ThreeOfAKind,
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            6 => HandType::FiveOfAKind,
            _ => HandType::HighCard
        }
    }

    fn get_with_joker(cards: &Vec<char>) -> Self {
        let mut checked_values = HashSet::new();
        checked_values.insert(&'J');
        let jokers = cards.iter().filter(|&n| *n == 'J').count() as u32;
        let mut pairs = 0;
        let mut three_of_a_kinds = 0;
        let mut four_of_a_kinds = 0;
        for card in cards {
            if checked_values.contains(card) {
                continue;
            }
            checked_values.insert(card);
            match cards.iter().filter(|&n| *n == *card).count() {
                2 => pairs += 1,
                3 => three_of_a_kinds +=1,
                4 => four_of_a_kinds += 1,
                5 => return HandType::FiveOfAKind,
                6 => return HandType::FiveOfAKind,
                _ => continue
            };
        }
        if four_of_a_kinds == 1 {
            HandType::repetitions_to_type(4 + jokers)
        }
        else if three_of_a_kinds == 1 && pairs == 1 {
            HandType::FullHouse
        } else if three_of_a_kinds == 1 {
            HandType::repetitions_to_type(3 + jokers)
        } else if pairs == 2 {
            if jokers == 1 {
                HandType::FullHouse
            }
            else {
                HandType::TwoPairs
            }
        } else if pairs == 1 {
            HandType::repetitions_to_type(jokers + 2)
        } else {
            HandType::repetitions_to_type(jokers + 1)
        }
    }
}

#[derive(PartialEq, Eq, Ord, Debug)]
struct Hand {
    hand_type: HandType,
    cards: Vec<char>,
    bid: u32
}

impl Hand {
    fn new(input: &str) -> Self {
        let (cards, bid) = input.trim().split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let cards: Vec<char> = cards.chars().collect();
        let hand_type = HandType::get(&cards);
        Self {
            hand_type,
            cards,
            bid
        }
    }

    fn new_with_joker(input: &str) -> Self {
        let (cards, bid) = input.trim().split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let cards: Vec<char> = cards.chars().collect();
        let hand_type = HandType::get_with_joker(&cards);
        Self {
            hand_type,
            cards,
            bid
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            Some(other.hand_type.cmp(&self.hand_type))
        } else {
            for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                if card1 == card2 {
                    continue
                }
                let card1_value = card_to_value(card1);
                let card2_value = card_to_value(card2);
                return Some(card1_value.cmp(&card2_value));
            }
            return None;
        }
    }
}

fn card_to_value(card: &char) -> u32 {
        match card {
            'T' => 10,
            'J' => unsafe{if j_meaning == JMeaning::Jack {11} else {1}},
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            number => number.to_string().parse().unwrap()
        }
}

fn solve_part_1(input: &str) -> u32 {
    let mut hands = vec![];
    for line in input.lines() {
        hands.push(Hand::new(line));
    }
    hands.sort();
    let mut result = 0;
    for (hand, order) in hands.iter().zip(1..=(hands.len() as u32)) {
        result += hand.bid * order;
    }
    result
}

fn solve_part_2(input: &str) -> u32 {
    unsafe{j_meaning = JMeaning::Joker};
    let mut hands = vec![];
    for line in input.lines() {
        hands.push(Hand::new_with_joker(line));
    }
    hands.sort();
    let mut result = 0;
    for (hand, order) in hands.iter().zip(1..=(hands.len() as u32)) {
        result += hand.bid * order;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let result = match task_part {
        1 => solve_part_1(&file_contents),
        2 => solve_part_2(&file_contents),
        _ => 0
    };
    println!("{}", result);
}
