use std::env;
use std::fs;
use std::collections::HashMap;

fn get_first_digit(input: &str) -> char {
    input.chars().find(|&character| character.is_numeric()).unwrap()
}

fn get_last_digit(input: &str) -> char {
    input.chars().rfind(|&character| character.is_numeric()).unwrap()
}

fn get_first_digit_spelled(input: &str, name_to_digit: &HashMap<&str, char>) -> char {
    for i in 0..input.len() {
        let first_character = input.chars().nth(i).unwrap();
        if first_character.is_numeric() {
            return first_character;
        }
        let string_slice = &input[i..];
        for (name, digit) in name_to_digit {
            if string_slice.starts_with(name) {
                return *digit;
            }
        }
    }
    println!("No number found in {}", input);
    return '0';
}

fn get_last_digit_spelled(input: &str, name_to_digit: &HashMap<&str, char>) -> char {
    for i in (0..input.len()).rev() {
        let first_character = input.chars().nth(i).unwrap();
        if first_character.is_numeric() {
            return first_character;
        }
        let string_slice = &input[i..];
        for (name, digit) in name_to_digit {
            if string_slice.starts_with(name) {
                return *digit;
            }
        }
    }
    println!("No number found in {}", input);
    return '0';
}

fn solve_part_1(input: &String) -> String {
    let mut numbers = Vec::<i32>::new();
    for line in input.lines() {
        let number_str = format!("{}{}", get_first_digit(line), get_last_digit(line));
        numbers.push(number_str.parse::<i32>().unwrap());
    }
    let sum: i32 = numbers.iter().sum();
    sum.to_string()
}

fn solve_part_2(input: &String, name_to_digit: &HashMap<&str, char>) -> String {
    let mut numbers = Vec::<i32>::new();
    for line in input.lines() {
        let number_str = format!("{}{}", get_first_digit_spelled(line, name_to_digit), get_last_digit_spelled(line, name_to_digit));
        numbers.push(number_str.parse::<i32>().unwrap());
    }
    let sum: i32 = numbers.iter().sum();
    sum.to_string()
}

fn main() {
    let mut name_to_digit: HashMap<&str, char> = HashMap::new();
    name_to_digit.insert("one", '1');
    name_to_digit.insert("two", '2');
    name_to_digit.insert("three", '3');
    name_to_digit.insert("four", '4');
    name_to_digit.insert("five", '5');
    name_to_digit.insert("six", '6');
    name_to_digit.insert("seven", '7');
    name_to_digit.insert("eight", '8');
    name_to_digit.insert("nine", '9');

    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<i32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let result = match task_part {
        1 => solve_part_1(&file_contents),
        2 => solve_part_2(&file_contents, &name_to_digit),
        _ => "UNKNOWN TASK PART".to_string()
    };
    println!("{}", result);
}
