use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West
        }
    }

    fn go(&self, coordinates: &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (coordinates.0.overflowing_sub(1).0, coordinates.1),
            Direction::South => (coordinates.0 + 1, coordinates.1),
            Direction::West => (coordinates.0, coordinates.1.overflowing_sub(1).0),
            Direction::East => (coordinates.0, coordinates.1 + 1)
        }
    }
}

#[derive(Clone)]
struct GridField {
    connected_directions: Vec<Direction>,
    coordinates: (usize, usize)
}

impl GridField {
    fn new(grid: &Vec<Vec<char>>, coordinates: &(usize, usize)) -> Self {
        let symbol = get_char(grid, coordinates);
        match symbol {
            '.' => Self {
                connected_directions: vec![],
                coordinates: coordinates.clone()
            },
            '-' => Self {
                connected_directions: vec![Direction::West, Direction::East],
                coordinates: coordinates.clone()
            },
            '|' => Self {
                connected_directions: vec![Direction::North, Direction::South],
                coordinates: coordinates.clone()
            },
            'L' => Self {
                connected_directions: vec![Direction::North, Direction::East],
                coordinates: coordinates.clone()
            },
            'J' => Self {
                connected_directions: vec![Direction::North, Direction::West],
                coordinates: coordinates.clone()
            },
            '7' => Self {
                connected_directions: vec![Direction::South, Direction::West],
                coordinates: coordinates.clone()
            },
            'F' => Self {
                connected_directions: vec![Direction::South, Direction::East],
                coordinates: coordinates.clone()
            },
            'S' => Self {
                connected_directions: vec![Direction::North, Direction::South, Direction::West, Direction::East],
                coordinates: coordinates.clone()
            },
            _ => panic!("Unexpected symbol")
        }
    }

    fn create(connected_directions: Vec<Direction>, coordinates: (usize, usize)) -> Self {
        Self {
            connected_directions,
            coordinates
        }
    }

    fn go_from(&self, from: &Direction) -> (Direction, (usize, usize)) { // returns next from and coordinates
        let to = self.connected_directions.iter().find(|direction| direction != &from ).unwrap();
        (to.opposite(), to.go(&self.coordinates))
    }
}

fn get_char(grid: &Vec<Vec<char>>, coordinates: &(usize, usize)) -> char {
    if coordinates.0 >= grid.len() || coordinates.1 >= grid[coordinates.0].len() {
        '.'
    } else {
        grid[coordinates.0][coordinates.1]
    }
}

fn get_nest_candidates(path: &Vec<GridField>) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for path_field in path {
        result.extend(get_neighbours(&path_field.coordinates));
    }
    result
}

fn get_neighbours(coordinates: &(usize, usize)) -> HashSet<(usize, usize)> {
    HashSet::from_iter(vec![
        (coordinates.0.overflowing_sub(1).0, coordinates.1),
        (coordinates.0 + 1, coordinates.1),
        (coordinates.0, coordinates.1.overflowing_sub(1).0),
        (coordinates.0, coordinates.1 + 1),
    ].into_iter())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(line.chars().collect())
    }
    result
}

fn check_if_nest_horizontally(coordinates: &(usize, usize), path: &Vec<GridField>) -> bool {
    let mut north_connections_to_west = 0;
    let mut south_connections_to_west = 0;
    for path_element in path {
        if path_element.coordinates.0 == coordinates.0 && path_element.coordinates.1 <  coordinates.1 {
            if path_element.connected_directions.contains(&Direction::North) {
                north_connections_to_west += 1;
            }
            if path_element.connected_directions.contains(&Direction::South) {
                south_connections_to_west += 1;
            }
        }
    }
    let north_south_connections_to_west = min(north_connections_to_west, south_connections_to_west);
    north_south_connections_to_west % 2 == 1
}

fn solve_part_1(grid: &Vec<Vec<char>>) -> u32 {
    let mut starting_position = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| *c == 'S') {
            starting_position = (i, j);
            break;
        }
    }
    let directions_to_search = vec![Direction::North, Direction::South, Direction::West, Direction::East];
    'a: for initial_direction in directions_to_search {
        let mut path_length = 2;
        let mut comming_from = initial_direction.opposite();
        let mut current_location = initial_direction.go(&starting_position);
        while get_char(grid, &current_location) != 'S' {
            let current_field = GridField::new(grid, &current_location);
            if !current_field.connected_directions.contains(&comming_from) {
                continue 'a;
            }
            (comming_from, current_location) = current_field.go_from(&comming_from);
            path_length += 1;
        }
        return path_length / 2;
    }
    0
}

fn solve_part_2(grid: &Vec<Vec<char>>) -> u32 {
    let mut starting_position = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|c| *c == 'S') {
            starting_position = (i, j);
            break;
        }
    }
    let directions_to_search = vec![Direction::North, Direction::South, Direction::West, Direction::East];
    let mut path_elements = vec![];
    'a: for initial_direction in directions_to_search {
        let mut comming_from = initial_direction.opposite();
        let mut current_location = initial_direction.go(&starting_position);
        path_elements.clear();
        while get_char(grid, &current_location) != 'S' {
            let current_field = GridField::new(grid, &current_location);
            if !current_field.connected_directions.contains(&comming_from) {
                continue 'a;
            }
            path_elements.push(current_field.clone());
            (comming_from, current_location) = current_field.go_from(&comming_from);
        }
        path_elements.push(GridField::create(vec![initial_direction, comming_from], starting_position) );
        break;
    }

    let mut nest_candidates = get_nest_candidates(&path_elements);
    let mut checked_fields = HashSet::from_iter(path_elements.iter().map(|path_element| path_element.coordinates));
    let mut possible_nest_locations = 0;
    nest_candidates = nest_candidates.difference(&checked_fields).map(|element| *element).collect();
    while !nest_candidates.is_empty() {
        let mut new_nest_candiadtes = HashSet::new();
        for candidate in &nest_candidates {
            if check_if_nest_horizontally(&candidate, &path_elements) {
                possible_nest_locations += 1;
                new_nest_candiadtes.extend(get_neighbours(&candidate));
                checked_fields.insert(*candidate);
            }
        }
        nest_candidates = new_nest_candiadtes.difference(&checked_fields).map(|element| *element).collect();
    }

    possible_nest_locations
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let grid = parse_input(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&grid),
        2 => solve_part_2(&grid),
        _ => 0
    };
    println!("{}", result);
}
