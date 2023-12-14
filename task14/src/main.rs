use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum RockType {
    Round(usize),
    Cube(usize)
}

impl RockType {
    fn get_weight(&self, column_height: &usize) -> usize {
        match self {
            RockType::Round(position) => column_height - position,
            RockType::Cube(_) => 0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct RockMap {
    rock_columns: Vec<Vec<RockType>>,
    rock_rows: Vec<Vec<RockType>>,
    column_height: usize,
    row_length: usize
}

impl RockMap {
    fn new(input: &str) -> Self {
        let row_length = input.find('\n').unwrap();
        let column_height = input.chars().filter(|c| *c == '\n').count();
        let mut rock_columns = vec![vec![]; row_length];
        let rock_rows = vec![vec![]; column_height];
        for (row, line) in input.lines().enumerate() {
            for (column, c) in line.chars().enumerate() {
                match c {
                    '#' => rock_columns[column].push(RockType::Cube(row)),
                    'O' => rock_columns[column].push(RockType::Round(row)),
                    _ => {}
                }
            }
        }
        Self {
            rock_columns,
            rock_rows,
            column_height,
            row_length
        }
    }

    fn roll_north(&mut self) {
        self.rock_rows = vec![vec![]; self.column_height];
        for (column_index, column) in &mut self.rock_columns.iter_mut().enumerate() {
            let mut first_available_spot = 0;
            for rock in column {
                match rock {
                    RockType::Round(location) => {
                        *location = first_available_spot;
                        first_available_spot += 1;
                        self.rock_rows[*location].push(RockType::Round(column_index));
                    },
                    RockType::Cube(location) => {
                        first_available_spot = *location + 1;
                        self.rock_rows[*location].push(RockType::Cube(column_index));
                    }
                }
                
            }
        }
    }

    fn roll_west(&mut self) {
        self.rock_columns = vec![vec![]; self.row_length];
        for (row_index, row) in &mut self.rock_rows.iter_mut().enumerate() {
            let mut first_available_spot = 0;
            for rock in row {
                match rock {
                    RockType::Round(location) => {
                        *location = first_available_spot;
                        first_available_spot += 1;
                        self.rock_columns[*location].push(RockType::Round(row_index));
                    },
                    RockType::Cube(location) => {
                        first_available_spot = *location + 1;
                        self.rock_columns[*location].push(RockType::Cube(row_index));
                    }
                }
                
            }
        }
    }

    fn roll_south(&mut self) {
        self.rock_rows = vec![vec![]; self.column_height];
        for (column_index, column) in &mut self.rock_columns.iter_mut().enumerate() {
            let mut first_available_spot = self.column_height - 1;
            for rock in column.iter_mut().rev() {
                match rock {
                    RockType::Round(location) => {
                        *location = first_available_spot;
                        first_available_spot = first_available_spot.overflowing_sub(1).0;
                        self.rock_rows[*location].push(RockType::Round(column_index));
                    },
                    RockType::Cube(location) => {
                        first_available_spot = location.overflowing_sub(1).0;
                        self.rock_rows[*location].push(RockType::Cube(column_index));
                    }
                }
                
            }
        }
    }

    fn roll_east(&mut self) {
        self.rock_columns = vec![vec![]; self.row_length];
        for (row_index, row) in &mut self.rock_rows.iter_mut().enumerate() {
            let mut first_available_spot = self.row_length - 1;
            for rock in row.iter_mut().rev() {
                match rock {
                    RockType::Round(location) => {
                        *location = first_available_spot;
                        first_available_spot = first_available_spot.overflowing_sub(1).0;
                        self.rock_columns[*location].push(RockType::Round(row_index));
                    },
                    RockType::Cube(location) => {
                        first_available_spot = location.overflowing_sub(1).0;
                        self.rock_columns[*location].push(RockType::Cube(row_index));
                    }
                }
                
            }
        }
    }

    fn perform_roll_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn calculate_weight(&self) -> usize {
        self.rock_columns.iter().fold(0, |sum, column| sum + column.iter().fold(0, |sum, rock| sum + rock.get_weight(&self.column_height)))
    }
}

fn solve_part_1(rock_map: &mut RockMap) -> usize {
    rock_map.roll_north();
    rock_map.calculate_weight()
}

fn solve_part_2(rock_map: &mut RockMap) -> usize {
    let mut configurations_encountered = HashMap::new(); // map to first iteration encountered
    let mut iteration = 0;
    let max_iteration = 1000000000;
    while iteration < max_iteration {
        rock_map.perform_roll_cycle();
        if let Some(cycle_start) = configurations_encountered.insert(rock_map.clone(), iteration) {
            let cycle_length = iteration - cycle_start;
            while iteration < max_iteration {
                iteration += cycle_length;
            }
            iteration -= cycle_length;
            configurations_encountered.clear();
        }
        iteration += 1;
    }
    rock_map.calculate_weight()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut rock_map = RockMap::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&mut rock_map),
        2 => solve_part_2(&mut rock_map),
        _ => 0
    };
    println!("{}", result);
}
