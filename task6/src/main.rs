use std::env;
use std::fs;

fn get_times_distances(input: &str) -> (Vec<f64>, Vec<f64>) {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let mut times = vec![];
    let (_, time_line) = time_line.split_once(':').unwrap();
    let time_line = time_line.trim();
    for time_string in time_line.split(' ') {
        if !time_string.is_empty() {
            times.push(time_string.parse().unwrap());
        }
    }
    let mut distances = vec![];
    let (_, distance_line) = distance_line.split_once(':').unwrap();
    let distance_line = distance_line.trim();
    for distance_string in distance_line.split(' ') {
        if !distance_string.is_empty() {
            distances.push(distance_string.parse().unwrap());
        }
    }
    (times, distances)
}

fn solve_quadratic_equation(b: f64, c: f64) -> (f64, f64) {
    let square_root_of_delta = (b.powf(2.) - 4. * c).sqrt();
    ((-b - square_root_of_delta) / 2., (-b + square_root_of_delta) / 2.)
}

fn solve_part_1(times: &Vec<f64>, distances: &Vec<f64>) -> f64 {
    let mut result = 1.;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let (x1, x2) = solve_quadratic_equation(-time, *distance);
        let x1 = if x1.fract() == 0. {x1 + 1.} else {x1};
        let x2 = if x2.fract() == 0. {x2 - 1.} else {x2};
        result *= x2.floor() - x1.ceil() + 1.
    }
    result
}

fn solve_part_2(times: &Vec<f64>, distances: &Vec<f64>) -> f64 {
    let mut time_string = String::new();
    let mut distance_string = String::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        time_string.push_str(&time.to_string());
        distance_string.push_str(&distance.to_string());
    }
    let time = time_string.parse::<f64>().unwrap();
    let distance = distance_string.parse::<f64>().unwrap();
    let (x1, x2) = solve_quadratic_equation(-time, distance);
    let x1 = if x1.fract() == 0. {x1 + 1.} else {x1};
    let x2 = if x2.fract() == 0. {x2 - 1.} else {x2};
    x2.floor() - x1.ceil() + 1.
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<i32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let (times, distances) = get_times_distances(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&times, &distances),
        2 => solve_part_2(&times, &distances),
        _ => -1.
    };
    println!("{}", result);
}
