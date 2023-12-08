use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

struct Intersection {
    left: String,
    right: String
}

#[derive(Clone)]
struct PathToZ {
    to_first: u64,
    distances_to_next: Vec<u64>,
    // using cache like that takes a lot of memory
    // known_distances: Vec<(u64, usize)> // distance, last step index
    last_known_distance: (u64, usize) // distance, last step index
}

impl PathToZ {
    fn new(start: &str, directions: &str, map: &HashMap<String, Intersection>) -> Self {
        let mut current_position = start;
        let mut steps = 0;
        let mut first_found = false;
        let mut z_visited = HashSet::new();
        let mut to_first = 0;
        let mut distances_to_next = vec![];
        loop {
            for direction in directions.chars() {
                current_position = match direction {
                    'L' => &map[current_position].left,
                    'R' => &map[current_position].right,
                    _ => panic!()
                };
                steps += 1;
                if current_position.ends_with('Z') {
                    if !first_found {
                        first_found = true;
                        to_first = steps;
                    }
                    else {
                        distances_to_next.push(steps);
                        let distances_to_next_len = distances_to_next.len();
                        if !z_visited.insert(current_position) {
                            // return PathToZ{to_first, distances_to_next, known_distances: vec![(to_first, distances_to_next_len - 1)]};
                            return PathToZ{to_first, distances_to_next, last_known_distance: (to_first, distances_to_next_len - 1)};
                        }
                    }
                    steps = 0;
                }
            }
        }
    }

    fn reaches_z(&mut self, steps: u64) -> bool {
        loop {
            let last_distance = self.last_known_distance.0;
            if steps < last_distance {
                return false;
            }
            else if steps == last_distance {
                return true;
            }
            let current_index = (self.last_known_distance.1 + 1) % self.distances_to_next.len();
            // self.known_distances.push((last_distance + self.distances_to_next[current_index], current_index));
            self.last_known_distance = (last_distance + self.distances_to_next[current_index], current_index);
        }
    }
}

fn parse_input(input: &str) -> (&str, HashMap<String, Intersection>) {
    let (directions, mapping_part) = input.split_once("\n\n").unwrap();
    let mut mapping = HashMap::new();
    for line in mapping_part.lines() {
        let (starting_point, destinations) = line.split_once(" = ").unwrap();
        let destinations = destinations.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
        let (left, right) = destinations.split_once(", ").unwrap();
        let starting_point = starting_point.to_string();
        let left = left.to_string();
        let right = right.to_string();
        mapping.insert(starting_point, Intersection{left, right});
    }

    (directions, mapping)
}

fn count_steps(start: &str, finish: &str, directions: &str, map: &HashMap<String, Intersection>) -> u64 {
    let mut current_position = start;
    let mut steps = 0;
    loop {
        for direction in directions.chars() {
            current_position = match direction {
                'L' => &map[current_position].left,
                'R' => &map[current_position].right,
                _ => panic!()
            };
            steps += 1;
            if current_position == finish {
                return steps;
            }
        }
    }
}

fn count_steps_multi_input(starts: Vec<String>, directions: &str, map: &HashMap<String, Intersection>) -> u64 {
    let mut paths_to_z = HashMap::new();
    for start in &starts {
        paths_to_z.insert(start, PathToZ::new(start, directions, map));
    }
    
    let reference_distance = (*paths_to_z.values().next().unwrap()).clone();
    let mut steps: u64 = reference_distance.to_first;
    loop {
        for distance_to_next in &reference_distance.distances_to_next {
            let mut all_good = true;
            for (_, path_to_z) in &mut paths_to_z {
                if !path_to_z.reaches_z(steps) {
                    all_good = false;
                    break;
                }
            }
            if all_good {
                return steps;
            }
            steps += distance_to_next;
        }
    }
}

fn solve_part_1(directions: &str, map: &HashMap<String, Intersection>) -> u64 {
    count_steps("AAA", "ZZZ", directions, map)
}

fn solve_part_2(directions: &str, map: &HashMap<String, Intersection>) -> u64 {
    let starts = map.keys().filter(|position| position.ends_with('A')).map(|s| s.clone()).collect();
    count_steps_multi_input(starts, directions, map)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let (directions, map) = parse_input(&file_contents);
    let result = match task_part {
        1 => solve_part_1(directions, &map),
        2 => solve_part_2(directions, &map),
        _ => 0
    };
    println!("{}", result);
}
