use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Index;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coordinates {
    x: i64,
    y: i64
}

impl Coordinates {
    fn new(x: i64, y: i64) -> Self {
        Self {x, y}
    }

    fn neighbours(&self) -> Vec<Self> {
        vec![
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y + 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x, self.y - 1),
        ]
    }
}

struct Map {
    map: Vec<Vec<char>>,
    rows: i64,
    columns: i64,
    starting_position: Coordinates,
    wrapping: bool
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map: Vec<Vec<char>> = vec![];
        let mut starting_position = Coordinates::new(0, 0);
        for (y, line) in input.lines().enumerate() {
            if let Some(x) = line.chars().position(|c| c == 'S') {
                starting_position = Coordinates::new(x as i64, y as i64);
            }
            map.push(line.chars().collect());
        }
        let rows = map.len() as i64;
        let columns = map[0].len() as i64;
        Self {
            map,
            rows,
            columns,
            starting_position,
            wrapping: false
        }
    }

    fn is_reachable(&self, coordinates: &Coordinates) -> bool {
        if !self.wrapping {
            coordinates.x >= 0 && coordinates.y >= 0 && coordinates.x < self.columns && coordinates.y < self.rows && self[coordinates] != '#'
        } else {
            self[coordinates] != '#'
        }
    }

    fn get_neighbours(&self, coordinates: &Coordinates) -> Vec<Coordinates> {
        coordinates.neighbours().iter().filter(|coordinates| self.is_reachable(coordinates)).map(|x| x.clone()).collect()
    }

    fn reachable_in_steps(&self, steps: usize, visualize: bool) -> usize {
        let mut reachable_even = 1; // starting position is reachable
        let mut reachable_odd = 0;
        let mut visited_coordinates = HashSet::new();
        visited_coordinates.insert(self.starting_position.clone());
        let mut frontier = vec![self.starting_position.clone()];
        let mut repetitions_visited = HashSet::new();
        repetitions_visited.insert(self.starting_position.clone());
        for i in 1..steps + 1 {
            let mut new_frontier = vec![];
            for current_coordinates in frontier {
                for neighbour in self.get_neighbours(&current_coordinates) {
                    if visited_coordinates.insert(neighbour.clone()) {
                        new_frontier.push(neighbour);
                        if i % 2 == 0 {
                            reachable_even += 1;
                        } else {
                            reachable_odd += 1
                        }
                    }
                }
            }
            frontier = new_frontier;
            if frontier.is_empty() {
                break;
            }
        }
        if visualize
        {
            let mut grid = self.map.clone();
            for coordinates in visited_coordinates {
                if self[&coordinates] != '#' &&
                    steps as i64 % 2 == (&coordinates.x + &coordinates.y) % 2 &&
                    coordinates.y < self.rows && coordinates.x < self.columns && coordinates.x >= 0 && coordinates.y >= 0 {
                        grid[coordinates.y as usize][coordinates.x as usize] = 'O';
                }
            }
            for line in grid {
                println!("{}", line.iter().collect::<String>());
            }
        }
        if steps % 2 == 0 {
            reachable_even
        } else {
            reachable_odd
        }
    }
}

impl Index<&Coordinates> for Map {
    type Output = char;
    fn index(&self, coordinates: &Coordinates) -> &Self::Output {
        if self.wrapping {
            &self.map[coordinates.y.rem_euclid(self.rows) as usize][coordinates.x.rem_euclid(self.columns) as usize]
        } else {
            &self.map[coordinates.y as usize][coordinates.x as usize]
        }
    }
}

fn solve_part_1(map: &Map, steps: usize, visualize: bool) -> usize {
    map.reachable_in_steps(steps, visualize)
}

fn solve_part_2(map: &mut Map, steps: usize, visualize: bool) -> usize {
    map.wrapping = true;
    map.reachable_in_steps(steps, visualize)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let steps = args[3].parse::<usize>().unwrap();
    let visualize: bool = args.get(4).unwrap_or(&"false".to_string()).parse().unwrap();
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut map = Map::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&map, steps, visualize),
        2 => solve_part_2(&mut map, steps, visualize),
        _ => 0
    };
    println!("{}", result);
}

// To solve part 2, with visualization you can see
// - it's possible to reach all the edges of initial map with 65 steps
// - to reach edges of extended map (input2.txt) you need 196 steps
// - calculating number of possible end locations for these inputs and next ones with interval 131 you get series 3682, 32768, 90820, 177838, 293822
// - this series can be extrapolated with formula 3562 - 14363 n + 14483 n^2
// - input = 65 + 202300 * 131
// - by substituting n in the formula with 202301 you get the final result