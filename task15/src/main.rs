use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u64
}

fn calculate_hash(input: &str, lookup_table: &mut HashMap<String, u8>) -> u8 {
    if lookup_table.contains_key(input) {
        return lookup_table[input];
    }
    let mut hash: u8 = 0;
    for c in input.chars().map(|c| c as u8) {
        hash = hash.wrapping_add(c);
        hash = hash.wrapping_mul(17);
    }
    lookup_table.insert(input.to_string(), hash);
    hash
}

fn solve_part_1(input: &str) -> u64 {
    let mut result = 0;
    let mut lookup_table = HashMap::<String, u8>::new();
    for command in input.split(',') {
        result += calculate_hash(command.trim(), &mut lookup_table) as u64;
    }
    result
}

fn solve_part_2(input: &str) -> u64 {
    let mut result = 0;
    let mut lookup_table = HashMap::<String, u8>::new();
    let mut boxes = vec![vec![]; 256];
    for command in input.split(',') {
        if command.contains('-') {
            let label = &command.trim()[..command.len() - 1];
            let box_number = calculate_hash(label, &mut lookup_table) as usize;
            boxes[box_number].retain(|lens: &Lens| lens.label != label);
        } else {
            let (label, focal_length) = command.trim().split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            let box_number = calculate_hash(label, &mut lookup_table) as usize;
            if let Some(lens) = boxes[box_number].iter_mut().find(|lens: &&mut Lens| lens.label == label) {
                lens.focal_length = focal_length;
            } else {
                boxes[box_number].push(Lens{label: label.to_string(), focal_length});
            }
        }
    }
    for (i, box_object) in boxes.iter().enumerate() {
        for (j, lens) in box_object.iter().enumerate() {
            result += (i + 1) as u64 * (j + 1) as u64 * lens.focal_length;
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let result = match task_part {
        1 => solve_part_1(&file_contents),
        2 => solve_part_2(&file_contents),
        _ => 0
    };
    println!("{}", result);
}
