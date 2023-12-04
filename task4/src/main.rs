use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

struct Scratchcard {
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
    _card_number: i32
}

impl Scratchcard {
    fn new(input: &str) -> Self {
        let (card_number_part, numbers_part) = input.split_once(": ").unwrap();
        let (_, card_number_str) = card_number_part.split_once(' ').unwrap();
        let _card_number = card_number_str.trim().parse::<i32>().unwrap();
        let (winning_set, own_set) = numbers_part.split_once(" | ").unwrap();
        let mut winning_numbers = vec![];
        let mut own_numbers = vec![];
        for number_str in winning_set.split(' ') {
            if !number_str.is_empty()
            {
                winning_numbers.push(number_str.parse::<i32>().unwrap());
            }
        }
        for number_str in own_set.split(' ') {
            if !number_str.is_empty()
            {
                own_numbers.push(number_str.parse::<i32>().unwrap());
            }
        }
        Self {
            winning_numbers,
            own_numbers,
            _card_number
        }
    }

    fn get_matching_numbers(&self) -> HashSet<i32> {
        let own_numbers_set: HashSet<i32> = HashSet::from_iter(self.own_numbers.iter().copied());
        let winning_numbers_set: HashSet<i32> = HashSet::from_iter(self.winning_numbers.iter().copied());
        own_numbers_set.intersection(&winning_numbers_set).copied().collect()
    }
}

fn get_set_points(set: &HashSet<i32>) -> i32 {
    let set_size = set.len() as u32;
    if set_size == 0 {
        0
    } else {
        2_i32.pow(set_size - 1)
    }
}

fn solve_part_1(cards: &Vec<Scratchcard>) -> i32 {
    let mut result = 0;
    for card in cards {
        let matching_numbers = card.get_matching_numbers();
        result += get_set_points(&matching_numbers);
    }
    result
}

fn solve_part_2(cards: &mut Vec<Scratchcard>) -> i32 {
    let mut result = 0;
    let mut prize_cards = HashMap::<usize, i32>::new();
    for (i, card) in cards.iter_mut().enumerate() {
        let card_ammount = 1 + prize_cards.get(&i).unwrap_or(&0);
        result += card_ammount;
        let matching_numbers = card.get_matching_numbers();
        for cards_won in 1..(matching_numbers.len() + 1) {
            *prize_cards.entry(i + cards_won).or_insert(0) += card_ammount;
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<i32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut cards = vec![];
    for line in file_contents.lines() {
        cards.push(Scratchcard::new(line));
    }
    let result = match task_part {
        1 => solve_part_1(&cards),
        2 => solve_part_2(&mut cards),
        _ => -1
    };
    println!("{}", result);
}
