use std::env;
use std::fs;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

extern crate blas_src;

const ERROR_MARGIN: f64 = 0.00001;

struct CoordinateWithSpeed {
    position: f64,
    velocity: f64
}

impl CoordinateWithSpeed {
    fn new(position: f64, velocity: f64) -> Self {
        Self {position, velocity}
    }
}

struct HailstonePositionWithVelocity {
    x: CoordinateWithSpeed,
    y: CoordinateWithSpeed,
    z: CoordinateWithSpeed
}

impl HailstonePositionWithVelocity {
    fn new(input: &str) -> Self {
        let (position_input, velocity_input) = input.split_once("@").unwrap();
        let position_coordinates: Vec<_> = position_input.split(',').collect();
        let x_position = position_coordinates[0].trim().parse().unwrap();
        let y_position = position_coordinates[1].trim().parse().unwrap();
        let z_position = position_coordinates[2].trim().parse().unwrap();
        let velocity_values: Vec<_> = velocity_input.split(',').collect();
        let x_velocity = velocity_values[0].trim().parse().unwrap();
        let y_velocity = velocity_values[1].trim().parse().unwrap();
        let z_velocity = velocity_values[2].trim().parse().unwrap();
        Self {
            x: CoordinateWithSpeed::new(x_position, x_velocity),
            y: CoordinateWithSpeed::new(y_position, y_velocity),
            z: CoordinateWithSpeed::new(z_position, z_velocity)
        }
    }

    fn find_x_y_intersection(&self, other: &Self) -> Option<(f64, f64, f64, f64)> { // (common_x, common_y, time of crossing for self, time of crossing for other)
        // we ignore the case where x speed is 0
        let self_velocity_quotient = self.y.velocity / self.x.velocity;
        let other_velocity_quotient = other.y.velocity / other.x.velocity;
        if self_velocity_quotient == other_velocity_quotient {
            // we ignore the case where the lines overlap
            return None;
        }
        let x = (other.y.position - self.y.position + self_velocity_quotient * self.x.position - other_velocity_quotient * other.x.position) / (self_velocity_quotient - other_velocity_quotient);
        let t1 = (x - self.x.position) / self.x.velocity;
        let t2 = (x - other.x.position) / other.x.velocity;
        let y = self.y.position + self.y.velocity * t1;
        Some((x, y, t1, t2))
    }
}

fn solve_for_starting_velocity(hailstones: &Vec<HailstonePositionWithVelocity>) -> (f64, f64, f64) {
    let mut coefficients = vec![];
    let mut results = vec![];
    for hailstone in hailstones {
        coefficients.push([hailstone.y.velocity, -hailstone.x.velocity, 0., -hailstone.y.position, hailstone.x.position, 0., -1., 0., 1., 0., 0., 0.]);
        results.push(hailstone.x.position * hailstone.y.velocity - hailstone.y.position * hailstone.x.velocity);
        coefficients.push([hailstone.z.velocity, 0., -hailstone.x.velocity, -hailstone.z.position, 0., hailstone.x.position, 0., -1., 0., 0., 1., 0.]);
        results.push(hailstone.x.position * hailstone.z.velocity - hailstone.z.position * hailstone.x.velocity);
        coefficients.push([0., hailstone.z.velocity, -hailstone.y.velocity, 0., -hailstone.z.position, hailstone.y.position, 0., 0., 0., -1., 0., 1.]);
        results.push(hailstone.y.position * hailstone.z.velocity - hailstone.z.position * hailstone.y.velocity);
    }
    for input in coefficients.iter().zip(results.iter()).combinations(12) {
        let matrix_rows: Vec<_> = input.iter().map(|(input_vector, _)| (*input_vector).clone()).collect();
        let result_vector: Vec<_> = input.iter().map(|(_, result)| **result).collect();
        let coefficient_array: Array2<f64> = matrix_rows.try_into().unwrap();
        let result_array: Array1<f64> = result_vector.try_into().unwrap();

        let solution = coefficient_array.solve(&result_array);
        if let Ok(variables) = solution {
            if (variables[3].round() - variables[3]).abs() < ERROR_MARGIN &&
                (variables[4].round() - variables[4]).abs() < ERROR_MARGIN &&
                (variables[5].round() - variables[5]).abs() < ERROR_MARGIN {
                    return (variables[3].round(), variables[4].round(), variables[5].round());
            }
        }
    }
    panic!("Starting velocity not found");
}

fn solve_for_starting_position(hailstones: &Vec<HailstonePositionWithVelocity>) -> (f64, f64, f64) {
    let mut coefficients = vec![];
    let mut results = vec![];
    let (vx, vy, vz) = solve_for_starting_velocity(hailstones);

    for hailstone in hailstones {
        if hailstone.x.velocity != vx && hailstone.y.velocity != vy {
            let helper_1 = hailstone.y.velocity - vy;
            let helper_2 = vx - hailstone.x.velocity;
            coefficients.push([helper_1, helper_2, 0.]);
            results.push(hailstone.x.position * helper_1 + hailstone.y.position * helper_2);
        }
        if hailstone.x.velocity != vx && hailstone.z.velocity != vz {
            let helper_1 = hailstone.z.velocity - vz;
            let helper_2 = vx - hailstone.x.velocity;
            coefficients.push([helper_1, 0., helper_2]);
            results.push(hailstone.x.position * helper_1 + hailstone.z.position * helper_2);
        }
        
        if hailstone.y.velocity != vy && hailstone.z.velocity != vz {
            let helper_1 = hailstone.z.velocity - vz;
            let helper_2 = vy - hailstone.y.velocity;
            coefficients.push([0., helper_1, helper_2]);
            results.push(hailstone.y.position * helper_1 + hailstone.z.position * helper_2);
        }
    }

    for input in coefficients.iter().zip(results.iter()).combinations(3) {
        let matrix_rows: Vec<_> = input.iter().map(|(input_vector, _)| (*input_vector).clone()).collect();
        let result_vector: Vec<_> = input.iter().map(|(_, result)| **result).collect();
        let coefficient_array: Array2<f64> = matrix_rows.try_into().unwrap();
        let result_array: Array1<f64> = result_vector.try_into().unwrap();

        let solution = coefficient_array.solve(&result_array);
        if let Ok(variables) = solution {
            if (variables[0].round() - variables[0]).abs() < ERROR_MARGIN &&
                (variables[1].round() - variables[1]).abs() < ERROR_MARGIN &&
                (variables[2].round() - variables[2]).abs() < ERROR_MARGIN {
                    return (variables[0].round(), variables[1].round(), variables[2].round());
            }
        }
    }
    panic!("Starting position not found");
}

fn solve_part_1(hailstones: &Vec<HailstonePositionWithVelocity>, min_position: f64, max_position: f64) -> usize {
    let mut result = 0;
    for hailstone_pair in hailstones.iter().combinations(2) {
        let intersection = hailstone_pair[0].find_x_y_intersection(hailstone_pair[1]);
        if let Some((x, y, t1, t2)) = intersection {
            if x >= min_position && x <= max_position &&
                y >= min_position && y <= max_position &&
                t1 > 0. && t2 > 0. {
                    result += 1;
            }
        }
    }
    result
}

fn solve_part_2(hailstones: &Vec<HailstonePositionWithVelocity>) -> usize {
    let (x, y, z) = solve_for_starting_position(hailstones);
    (x + y + z) as usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let mut min_position = 0.;
    let mut max_position = 0.;
    if task_part == 1 {
        min_position = args[3].parse().unwrap();
        max_position = args[4].parse().unwrap();
    }
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut hailstones = vec![];
    for line in file_contents.lines() {
        hailstones.push(HailstonePositionWithVelocity::new(&line));
    }
    let result = match task_part {
        1 => solve_part_1(&hailstones, min_position, max_position),
        2 => solve_part_2(&hailstones),
        _ => 0
    };
    println!("{}", result);
}
