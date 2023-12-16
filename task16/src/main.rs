use std::collections::HashSet;
use std::env;
use std::fs;

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
    fn reflect(&self, field_contents: char) -> Vec<Self> {
        match self {
            Direction::Up => {
                match field_contents {
                    '/' => vec![Direction::Right],
                    '\\' => vec![Direction::Left],
                    '|' => vec![Direction::Up],
                    '-' => vec![Direction::Left, Direction::Right],
                    _ => vec![Direction::Up]
                }
            },
            Direction::Down => {
                match field_contents {
                    '/' => vec![Direction::Left],
                    '\\' => vec![Direction::Right],
                    '|' => vec![Direction::Down],
                    '-' => vec![Direction::Left, Direction::Right],
                    _ => vec![Direction::Down]
                }
            },
            Direction::Left => {
                match field_contents {
                    '/' => vec![Direction::Down],
                    '\\' => vec![Direction::Up],
                    '|' => vec![Direction::Up, Direction::Down],
                    '-' => vec![Direction::Left],
                    _ => vec![Direction::Left]
                }
            },
            Direction::Right => {
                match field_contents {
                    '/' => vec![Direction::Up],
                    '\\' => vec![Direction::Down],
                    '|' => vec![Direction::Up, Direction::Down],
                    '-' => vec![Direction::Right],
                    _ => vec![Direction::Right]
                }
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct BeamLocation {
    coordinates: Coordinates,
    direction: Direction
}

impl BeamLocation {
    fn new(coordinates: Coordinates, direction: Direction) -> Self {
        BeamLocation {
            coordinates,
            direction
        }
    }

    fn procede(self, field_contents: char, map_width: usize, map_height: usize) -> Vec<Self> {
        let mut result = vec![];
        for new_direction in self.direction.reflect(field_contents) {
            let new_location = match new_direction {
                Direction::Up => Coordinates::new(self.coordinates.x, self.coordinates.y.wrapping_sub(1)),
                Direction::Down => Coordinates::new(self.coordinates.x, self.coordinates.y + 1),
                Direction::Left => Coordinates::new(self.coordinates.x.wrapping_sub(1), self.coordinates.y),
                Direction::Right => Coordinates::new(self.coordinates.x + 1, self.coordinates.y),
            };
            if new_location.check_bounds(map_width, map_height) {
                result.push(BeamLocation::new(new_location, new_direction));
            }
        }
        result
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(line.chars().collect())
    }
    result
}

fn solve_part_1(map: &Vec<Vec<char>>, beam_location: BeamLocation) -> usize {
    let mut current_beams = vec![beam_location];
    let mut visited_coordinates = HashSet::new();
    let mut beams_encountered = HashSet::new();
    let map_height = map.len();
    let map_width = map[0].len();
    while let Some(beam) = current_beams.pop() {
        visited_coordinates.insert(beam.coordinates.clone());
        if !beams_encountered.insert(beam.clone()) {
            continue;
        }
        let field_contents = map[beam.coordinates.y][beam.coordinates.x];
        current_beams.append(&mut beam.procede(field_contents, map_width, map_height));
    }
    visited_coordinates.len()
}

fn solve_part_2(map: &Vec<Vec<char>>) -> usize {
    let map_height = map.len();
    let map_width = map[0].len();
    let mut results = vec![];
    for i in 0..map_width {
        results.push(solve_part_1(map, BeamLocation::new(Coordinates::new(i, 0), Direction::Down)));
        results.push(solve_part_1(map, BeamLocation::new(Coordinates::new(i, map_height-1), Direction::Up)));
    }
    for i in 0..map_height {
        results.push(solve_part_1(map, BeamLocation::new(Coordinates::new(0, i), Direction::Right)));
        results.push(solve_part_1(map, BeamLocation::new(Coordinates::new(map_height-1, i), Direction::Left)));
    }
    *results.iter().max().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let map = parse(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&map, BeamLocation::new(Coordinates::new(0, 0), Direction::Right)),
        2 => solve_part_2(&map),
        _ => 0
    };
    println!("{}", result);
}
