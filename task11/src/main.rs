use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;

struct Coordinates {
    x: i64,
    y: i64
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i64,
            y: y as i64
        }
    }

    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct SkyMap {
    fields: Vec<Vec<char>>,
    empty_columns: HashSet<usize>,
    empty_rows: HashSet<usize>
}

impl SkyMap {
    fn new(input: &str) -> Self {
        let mut fields: Vec<Vec<char>> = vec![];
        let mut empty_columns = HashSet::new();
        let mut empty_rows = HashSet::new();
        for line in input.lines() {
            fields.push(line.chars().collect());
        }

        for column_index in 0..fields[0].len() {
            if fields.iter().all(|row| row[column_index] == '.') {
                empty_columns.insert(column_index);
            }
        }

        for (row_index, row) in fields.iter().enumerate() {
            if row.iter().all(|field| *field == '.') {
                empty_rows.insert(row_index);
            }
        }

        Self {
            fields,
            empty_columns,
            empty_rows
        }
    }
}

fn solve_part_1(sky_map: &SkyMap) -> i64 {
    let mut galaxy_coordinates = vec![];
    let mut empty_rows_encountered = 0;
    for (y, row) in sky_map.fields.iter().enumerate() {
        if sky_map.empty_rows.contains(&y) {
            empty_rows_encountered += 1;
            continue;
        }
        let mut empty_columns_encountered = 0;
        for (x, character) in row.iter().enumerate() {
            if sky_map.empty_columns.contains(&x) {
                empty_columns_encountered += 1;
            }
            else if *character == '#' {
                galaxy_coordinates.push(Coordinates::new(x + empty_columns_encountered, y + empty_rows_encountered));
            }
        }
    }
    let mut result = 0;
    for combination in galaxy_coordinates.iter().combinations(2) {
        result += combination[0].manhattan_distance(combination[1]);
    }
    result
}

fn solve_part_2(sky_map: &SkyMap, expansion_factor: usize) -> i64 {
    let mut galaxy_coordinates = vec![];
    let mut empty_rows_encountered = 0;
    for (y, row) in sky_map.fields.iter().enumerate() {
        if sky_map.empty_rows.contains(&y) {
            empty_rows_encountered += 1;
            continue;
        }
        let mut empty_columns_encountered = 0;
        for (x, character) in row.iter().enumerate() {
            if sky_map.empty_columns.contains(&x) {
                empty_columns_encountered += 1;
            }
            else if *character == '#' {
                galaxy_coordinates.push(Coordinates::new(x + empty_columns_encountered * (expansion_factor - 1), y + empty_rows_encountered * (expansion_factor - 1)));
            }
        }
    }
    let mut result = 0;
    for combination in galaxy_coordinates.iter().combinations(2) {
        result += combination[0].manhattan_distance(combination[1]);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let expansion_factor = args[3].parse().unwrap();
    let file_contents = fs::read_to_string(file_path).unwrap();
    let sky_map = SkyMap::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&sky_map),
        2 => solve_part_2(&sky_map, expansion_factor),
        _ => -1
    };
    println!("{}", result);
}
