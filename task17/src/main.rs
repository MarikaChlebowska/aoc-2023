use std::collections::{HashSet, BTreeMap};
use std::env;
use std::fs;

type Map = Vec<Vec<usize>>;

enum Vehicle {
    Crucible,
    UltraCrucible
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coordinates {
    x: usize,
    y: usize
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }

    fn check_bounds(&self, map_width: usize, map_height: usize) -> bool {
        self.x < map_width && self.y < map_height
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn make_step(&self, position: &Coordinates, map_width: usize, map_height: usize) -> Option<Coordinates> {
        let next_coordinates = match self {
            Direction::Up => {
                Coordinates::new(position.x, position.y.wrapping_sub(1))
            },
            Direction::Down => {
                Coordinates::new(position.x, position.y + 1)
            },
            Direction::Left => {
                Coordinates::new(position.x.wrapping_sub(1), position.y)
            },
            Direction::Right => {
                Coordinates::new(position.x + 1, position.y)
            },
        };
        if next_coordinates.check_bounds(map_width, map_height) {
            Some(next_coordinates)
        }
        else {
            None
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Movement {
    position: Coordinates,
    last_direction: Direction,
    consecutive_steps: usize
}

impl Movement {
    fn new(position: &Coordinates, last_direction: Direction, consecutive_steps: usize) -> Self {
        Self {
            position: position.clone(),
            last_direction,
            consecutive_steps
        }
    }

    fn possible_next_steps(&self, vehicle: &Vehicle, map_width: usize, map_height: usize) -> Vec<Self> {
        let mut result = vec![];
        let possible_steps = match vehicle {
            Vehicle::Crucible => {
                let mut possible_steps = match &self.last_direction {
                    Direction::Up => vec![Direction::Left, Direction::Right],
                    Direction::Down => vec![Direction::Left, Direction::Right],
                    Direction::Left => vec![Direction::Up, Direction::Down],
                    Direction::Right => vec![Direction::Up, Direction::Down],
                };
                if self.consecutive_steps < 3 {
                    possible_steps.push(self.last_direction.clone());
                }
                possible_steps
            },
            Vehicle::UltraCrucible => {
                let mut possible_steps = vec![];
                if self.consecutive_steps < 10 {
                    possible_steps.push(self.last_direction.clone());
                }
                if self.consecutive_steps > 3 {
                    match &self.last_direction {
                        Direction::Up => possible_steps.extend(vec![Direction::Left, Direction::Right]),
                        Direction::Down => possible_steps.extend(vec![Direction::Left, Direction::Right]),
                        Direction::Left => possible_steps.extend(vec![Direction::Up, Direction::Down]),
                        Direction::Right => possible_steps.extend(vec![Direction::Up, Direction::Down]),
                    };
                }
                possible_steps
            }
        };
        for step in possible_steps {
            if let Some(position) = step.make_step(&self.position, map_width, map_height) {
                let consecutive_steps = if self.last_direction == step {
                    self.consecutive_steps + 1
                } else {
                    1
                };
                result.push(Movement::new(&position, step, consecutive_steps));
            }
        }
        result
    }
}

fn parse(input: &str) -> Map {
    let mut result = vec![];
    for line in input.lines() {
        result.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
    }
    result
}

fn find_shortest_path(map: &Map, vehicle: Vehicle, goal: &Coordinates, locations_to_check: &mut BTreeMap<usize, Vec<Movement>>) -> usize {
    let mut checked_movements = HashSet::new();
    loop {
        let (distance, locations) = locations_to_check.pop_first().unwrap();
        let current_location = locations.last().unwrap();
        if locations.len() > 1 {
            locations_to_check.insert(distance, locations[..locations.len() - 1].into());
        }
        if current_location.position == *goal {
            return distance;
        }
        if !checked_movements.contains(current_location) {
            checked_movements.insert(current_location.clone());
            for next_move in current_location.possible_next_steps(&vehicle, map[0].len(), map.len()) {
                let new_distance = distance + map[next_move.position.y][next_move.position.x];
                if locations_to_check.contains_key(&new_distance) {
                    locations_to_check.get_mut(&new_distance).unwrap().push(next_move);
                } else {
                    locations_to_check.insert(new_distance, vec![next_move]);
                }

            }
        }
    }
}

fn solve_part_1(map: &Map) -> usize {
    let mut locations_to_check = BTreeMap::new();
    locations_to_check.insert(0, vec![
        Movement::new(
            &Coordinates::new(0, 0),
            Direction::Right,
            0
        )
    ]);
    let goal = Coordinates::new(map[0].len() - 1, map.len() - 1);
    find_shortest_path(&map, Vehicle::Crucible, &goal, &mut locations_to_check)
}

fn solve_part_2(map: &Map) -> usize {
    let mut locations_to_check = BTreeMap::new();
    locations_to_check.insert(0, vec![
        Movement::new(
            &Coordinates::new(0, 0),
            Direction::Right,
            0
        ),
        Movement::new(
            &Coordinates::new(0, 0),
            Direction::Down,
            0
        )
    ]);
    let goal = Coordinates::new(map[0].len() - 1, map.len() - 1);
    find_shortest_path(&map, Vehicle::UltraCrucible, &goal, &mut locations_to_check)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let map = parse(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&map),
        2 => solve_part_2(&map),
        _ => 0
    };
    println!("{}", result);
}
