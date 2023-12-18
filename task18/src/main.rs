use std::env;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn new(input: &str) -> Self {
        match input.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction")
        }
    }

    fn new_from_colour(input: &char) -> Self {
        match input {
            '3' => Direction::Up,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '0' => Direction::Right,
            _ => panic!("Unknown direction")
        }
    }
}

#[derive(Clone, Debug)]
struct Coordinates {
    x: f64,
    y: f64
}

impl Coordinates {
    fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }
}

struct DigPlanPoint {
    direction: Direction,
    length: f64,
    colour: String
}

impl DigPlanPoint {
    fn new(input: &str) -> Self {
        let fields: Vec<_> = input.split(' ').collect();
        let direction = Direction::new(fields[0]);
        let length = fields[1].parse().unwrap();
        let colour = fields[2][1..fields[2].len()-1].to_string();
        Self {
            direction,
            length,
            colour
        }
    }

    fn get_numbers_from_colours(&self) -> Self {
        let direction = Direction::new_from_colour(&self.colour.chars().last().unwrap());
        let length = usize::from_str_radix(&self.colour[1..self.colour.len() - 1], 16).unwrap() as f64;
        Self {
            direction,
            length,
            colour: self.colour.clone()
        }
    }

    fn get_coordinates_after_move(&self, coordinates: Coordinates) -> Coordinates {
        match self.direction {
            Direction::Up => Coordinates::new(coordinates.x, coordinates.y + self.length),
            Direction::Down => Coordinates::new(coordinates.x, coordinates.y - self.length),
            Direction::Left => Coordinates::new(coordinates.x - self.length, coordinates.y),
            Direction::Right => Coordinates::new(coordinates.x + self.length, coordinates.y),
        }
    }
}

#[derive(PartialEq, Debug)]
enum CornerType {
    UpperRight,
    LowerRight,
    LowerLeft,
    UpperLeft
}

impl CornerType {
    fn new(previous_direction: Direction, next_direction: Direction) -> Self {
        if previous_direction == Direction::Up && next_direction == Direction::Left {
            CornerType::UpperRight
        } else if previous_direction == Direction::Up && next_direction == Direction::Right {
            CornerType::UpperLeft
        } else if previous_direction == Direction::Right && next_direction == Direction::Up {
            CornerType::LowerRight
        } else if previous_direction == Direction::Right && next_direction == Direction::Down {
            CornerType::UpperRight
        } else if previous_direction == Direction::Down && next_direction == Direction::Left {
            CornerType::LowerRight
        } else if previous_direction == Direction::Down && next_direction == Direction::Right {
            CornerType::LowerLeft
        } else if previous_direction == Direction::Left && next_direction == Direction::Up {
            CornerType::LowerLeft
        } else if previous_direction == Direction::Left && next_direction == Direction::Down {
            CornerType::UpperLeft
        } else {
            panic!("That's not a corner!");
        }
    }

    fn is_opposite(&self, other: &CornerType) -> bool {
        match self {
            CornerType::UpperRight => {
                other == &CornerType::LowerLeft
            },
            CornerType::LowerRight => {
                other == &CornerType::UpperLeft
            },
            CornerType::LowerLeft => {
                other == &CornerType::UpperRight
            },
            CornerType::UpperLeft => {
                other == &CornerType::LowerRight
            }
        }
    }

    fn get_outer_point(&self, point: &Coordinates, is_oblique: bool) -> Coordinates {
        match self {
            CornerType::UpperRight => {
                if is_oblique {
                    Coordinates::new(point.x + 0.5, point.y + 0.5)
                } else {
                    Coordinates::new(point.x - 0.5, point.y - 0.5)
                }
            },
            CornerType::LowerRight => {
                if is_oblique {
                    Coordinates::new(point.x + 0.5, point.y - 0.5)
                } else {
                    Coordinates::new(point.x - 0.5, point.y + 0.5)
                }
            },
            CornerType::LowerLeft => {
                if is_oblique {
                    Coordinates::new(point.x - 0.5, point.y - 0.5)
                } else {
                    Coordinates::new(point.x + 0.5, point.y + 0.5)
                }
            },
            CornerType::UpperLeft => {
                if is_oblique {
                    Coordinates::new(point.x - 0.5, point.y + 0.5)
                } else {
                    Coordinates::new(point.x + 0.5, point.y - 0.5)
                }
            }
        }
    }
}

fn generate_points(dig_plan: &Vec<DigPlanPoint>) -> Vec<Coordinates> {
    let mut result = vec![Coordinates::new(0.0, 0.0)];
    for dig_plan_point in &dig_plan[..dig_plan.len() - 1] {
        result.push(dig_plan_point.get_coordinates_after_move(result.last().unwrap().clone()))
    }
    result
}

fn find_right_most_in_top_row(points: &Vec<Coordinates>) -> usize {
    points.iter().enumerate().max_by(|(_, point1), (_, point2)| {if point1.y == point2.y {point1.x.partial_cmp(&point2.x).unwrap()} else {point1.y.partial_cmp(&point2.y).unwrap()}}).unwrap().0
}

fn generate_outer_points (dig_plan: &Vec<DigPlanPoint>) -> Vec<Coordinates> {
    let points = generate_points(dig_plan);
    let starting_index = find_right_most_in_top_row(&points);
    let mut previous_direction = dig_plan[starting_index].direction.clone();
    let mut is_oblique = true;
    let mut previous_corner = CornerType::UpperRight;
    let mut result = vec![CornerType::UpperRight.get_outer_point(&points[starting_index], true)];
    for (point, plan_point) in points.iter().zip(dig_plan.iter()).cycle().skip(starting_index + 1).take(dig_plan.len() - 1) {
        let corner = CornerType::new(previous_direction, plan_point.direction.clone());
        if previous_corner.is_opposite(&corner) {
            is_oblique = !is_oblique;
        }
        result.push(corner.get_outer_point(point, is_oblique));
        previous_corner = corner;
        previous_direction = plan_point.direction.clone()
    }
    result
}

fn calculate_area(points: &Vec<Coordinates>) -> i64 {
    let last_point = points.last().unwrap();
    let first_point = points.first().unwrap();
    let mut sum = last_point.x * first_point.y - last_point.y * first_point.x;
    for current_points in points.windows(2) {
        sum += current_points[0].x * current_points[1].y - current_points[0].y * current_points[1].x;
    }
    (sum.abs() / 2.) as i64
}

fn solve_part_1(dig_plan: &Vec<DigPlanPoint>) -> i64 {
    let outer_points = generate_outer_points(dig_plan);
    calculate_area(&outer_points)
}

fn solve_part_2(dig_plan: &Vec<DigPlanPoint>) -> i64 {
    let mut dig_plan_from_colours = vec![];
    for dig_plan_point in dig_plan {
        dig_plan_from_colours.push(dig_plan_point.get_numbers_from_colours());
    }
    let outer_points = generate_outer_points(&dig_plan_from_colours);
    calculate_area(&outer_points)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut dig_plan = vec![];
    for line in file_contents.lines() {
        dig_plan.push(DigPlanPoint::new(line));
    }
    let result = match task_part {
        1 => solve_part_1(&dig_plan),
        2 => solve_part_2(&dig_plan),
        _ => 0
    };
    println!("{}", result);
}
