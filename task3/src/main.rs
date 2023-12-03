use std::collections::HashSet;
use std::env;
use std::fs;

struct EngineGrid {
    grid: Vec<Vec<char>>,
    width: i32,
    height: i32
}

impl EngineGrid {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<char>> = vec![];
        for line in input.lines() {
            grid.push(line.chars().collect());
        }
        // there is an assumption that the grid is square
        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        EngineGrid {
            grid,
            width,
            height
        }
    }

    fn is_inside_grid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn get_neighbour_coordinates(&self, x: i32, y:i32) -> Vec<(i32, i32)> {
        let mut result = vec![];
        let neighbour_candidates = vec![(x-1, y-1), (x, y-1), (x+1, y-1), (x+1, y), (x+1, y+1), (x, y+1), (x-1, y+1), (x-1, y)];
        for neighbour_candidate in neighbour_candidates {
            if self.is_inside_grid(neighbour_candidate.0, neighbour_candidate.1) {
                result.push(neighbour_candidate);
            }
        }
        result
    }

    fn get_number_start(&self, x: i32, y: i32) -> (i32, i32) {
        let mut starting_x = x;
        while self.is_inside_grid(starting_x - 1, y) && self.grid[y as usize][(starting_x - 1) as usize].is_numeric() {
            starting_x -= 1;
        }
        (starting_x, y)
    }

    fn get_number(&self, x: i32, y: i32) -> i32 {
        let mut ending_x = x + 1;
        while self.is_inside_grid(ending_x, y) && self.grid[y as usize][ending_x as usize].is_numeric() {
            ending_x += 1;
        }
        let number_str: String = self.grid[y as usize][x as usize..ending_x as usize].iter().collect();
        number_str.parse::<i32>().unwrap()
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn solve_part_1(grid: &EngineGrid) -> i32 {
    let mut result = 0;
    let mut number_coordinates = HashSet::<(i32, i32)>::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if is_symbol(grid.grid[y as usize][x as usize]) {
                let neighbours = grid.get_neighbour_coordinates(x, y);
                for (x_neighbour, y_neighbour) in neighbours {
                    if grid.grid[y_neighbour as usize][x_neighbour as usize].is_numeric() {
                        number_coordinates.insert(grid.get_number_start(x_neighbour, y_neighbour));
                    }
                }
            }
        }
    }
    for (x, y) in number_coordinates {
        result += grid.get_number(x, y);
    }
    result
}

fn solve_part_2(grid: &EngineGrid) -> i32 {
    let mut result = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.grid[y as usize][x as usize] == '*' {
                let neighbours = grid.get_neighbour_coordinates(x, y);
                let mut number_coordinates = HashSet::<(i32, i32)>::new();
                for (x_neighbour, y_neighbour) in neighbours {
                    if grid.grid[y_neighbour as usize][x_neighbour as usize].is_numeric() {
                        number_coordinates.insert(grid.get_number_start(x_neighbour, y_neighbour));
                    }
                }
                if number_coordinates.len() == 2 {
                    let mut gear_ratio = 1;
                    for (x_number, y_number) in number_coordinates {
                        gear_ratio *= grid.get_number(x_number, y_number);
                    }
                    result += gear_ratio;
                }
            }
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<i32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let grid = EngineGrid::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&grid),
        2 => solve_part_2(&grid),
        _ => -1
    };
    println!("{}", result);
}
