use std::env;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];
    for line in input.lines() {
        let mut series = vec![];
        for number in line.split(' ') {
            series.push(number.parse().unwrap());
        }
        result.push(series);
    }
    result
}

fn calculate_difference(series: &Vec<i32>) -> (Vec<i32>, bool) { // (difference, is it constant?)
    let mut result = vec![];
    let mut constant = true;
    let first_difference = series[1] - series[0];
    let mut previous_number = series[0];
    for number in series.iter().skip(1) {
        let new_value = number - previous_number;
        result.push(new_value);
        if new_value != first_difference {
            constant = false;
        }
        previous_number = *number;
    }
    (result, constant)
}

fn calculate_next(series: &Vec<i32>) -> i32 {
    let mut differences = vec![series.clone()];
    loop {
        let (new_difference, constant) = calculate_difference(differences.last().unwrap());
        differences.push(new_difference);
        if constant {
            break;
        }
    }
    differences.iter().fold(0, |sum, vector| {sum + vector.last().unwrap()})
}

fn calculate_previous(series: &Vec<i32>) -> i32 {
    let mut differences = vec![series.clone()];
    loop {
        let (new_difference, constant) = calculate_difference(differences.last().unwrap());
        differences.push(new_difference);
        if constant {
            break;
        }
    }
    let mut even = false;
    differences.iter().skip(1).fold(*differences[0].first().unwrap(), |previous, vector| {even = !even; if even {previous - vector.first().unwrap()} else {previous + vector.first().unwrap()}})
}

fn solve_part_1(series_vector: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for series in series_vector {
        result += calculate_next(&series);
    }
    result
}

fn solve_part_2(series_vector: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for series in series_vector {
        result += calculate_previous(&series);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let series_vector = parse_input(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&series_vector),
        2 => solve_part_2(&series_vector),
        _ => 0
    };
    println!("{}", result);
}
