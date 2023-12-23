use std::collections::{HashMap, HashSet};
use std::cmp::max;
use std::env;
use std::fs;
use std::ops::Index;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
struct Coordinates {
    x: usize,
    y: usize
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x.wrapping_sub(1), self.y),
            Self::new(self.x, self.y.wrapping_sub(1)),
        ]
    }
}

struct Map {
    map: Vec<Vec<char>>,
    rows: usize,
    columns: usize,
    starting_position: Coordinates,
    end_position: Coordinates,
    slippery: bool
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            map.push(line.chars().collect());
        }
        let rows = map.len();
        let columns = map[0].len();
        let starting_position = Coordinates::new(map.first().unwrap().iter().position(|c| *c == '.').unwrap(), 0);
        let end_position = Coordinates::new(map.last().unwrap().iter().position(|c| *c == '.').unwrap(), rows - 1);
        Self {
            map,
            rows,
            columns,
            starting_position,
            end_position,
            slippery: true
        }
    }

    fn is_reachable(&self, coordinates: &Coordinates) -> bool {
        coordinates.x < self.columns && coordinates.y < self.rows && self[coordinates] != '#'
    }

    fn get_possible_steps(&self, coordinates: &Coordinates) -> Vec<Coordinates> {
        let possible_steps = if self.slippery {
            match self[coordinates] {
                '.' => coordinates.neighbours(),
                '^' => vec![Coordinates::new(coordinates.x, coordinates.y.wrapping_sub(1))],
                '>' => vec![Coordinates::new(coordinates.x + 1, coordinates.y)],
                'v' => vec![Coordinates::new(coordinates.x, coordinates.y + 1)],
                '<' => vec![Coordinates::new(coordinates.x.wrapping_sub(1), coordinates.y)],
                _ => vec![]
            }
        } else {
            coordinates.neighbours()
        };
        possible_steps.iter().filter(|coordinates| self.is_reachable(coordinates)).map(|x| x.clone()).collect()
    }

    fn find_longest_path_without_repetition(&self) -> usize {
        let mut paths_to_check_with_positions = vec![(HashSet::from([self.starting_position]), self.starting_position.clone())];
        let mut longest_path = 0;
        let mut position_to_reachable_cache: HashMap<Coordinates, Vec<Coordinates>> = HashMap::new();
        while let Some((mut visited, mut current_position)) = paths_to_check_with_positions.pop() {
            loop {
                if current_position == self.end_position {
                    longest_path = max(longest_path, visited.len() - 1); // starting position doesn't count
                    break;
                }
                let mut possible_steps = if position_to_reachable_cache.contains_key(&current_position) {
                    position_to_reachable_cache[&current_position].clone()
                } else {
                    let possible_steps = self.get_possible_steps(&current_position);
                    position_to_reachable_cache.insert(current_position, possible_steps.clone());
                    possible_steps
                };
                possible_steps.retain(|position| !visited.contains(position));
                if possible_steps.is_empty() {
                    break;
                } else if possible_steps.len() == 1 {
                    current_position = *possible_steps.first().unwrap();
                    visited.insert(current_position);
                } else {
                    for step in possible_steps.iter().skip(1) {
                        let mut visited_copy = visited.clone();
                        visited_copy.insert(*step);
                        paths_to_check_with_positions.push((visited_copy, *step));
                    }
                    current_position = *possible_steps.first().unwrap();
                    visited.insert(current_position);
                }
            }
        }
        longest_path
    }
}

impl Index<&Coordinates> for Map {
    type Output = char;
    fn index(&self, coordinates: &Coordinates) -> &Self::Output {
         &self.map[coordinates.y][coordinates.x]
    }
}

fn solve_part_1(map: &Map) -> usize {
    map.find_longest_path_without_repetition()
}

fn solve_part_2(map: &mut Map) -> usize {
    map.slippery = false;
    map.find_longest_path_without_repetition()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut map = Map::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&map),
        2 => solve_part_2(&mut map),
        _ => 0
    };
    println!("{}", result);
}
