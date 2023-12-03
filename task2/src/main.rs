use core::cmp::max;
use std::env;
use std::fs;

struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

impl Draw {
    fn parse(input: &str) -> Draw {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let color_inputs: Vec<&str> = input.split(", ").collect();
        for color_input in color_inputs {
            let (number, color) = color_input.split_once(' ').unwrap();
            match color {
                "red" => red += number.parse::<i32>().unwrap(),
                "green" => green += number.parse::<i32>().unwrap(),
                "blue" => blue += number.parse::<i32>().unwrap(),
                something_else => println!("Unknown color: {}", something_else)
            };
        }
        Draw {
            red,
            green,
            blue
        }
    }

    fn get_power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    index: i32,
    draws: Vec<Draw>
}

impl Game {
    fn parse(input: &str) -> Game {
        let (number_part, draws_part) = input.split_once(": ").unwrap();
        let (_, index) = number_part.split_once(' ').unwrap();
        let index = index.parse::<i32>().unwrap();
        let draw_inputs: Vec<&str> = draws_part.split("; ").collect();
        let mut draws = Vec::<Draw>::new();
        for draw_input in draw_inputs {
            draws.push(Draw::parse(draw_input))
        }

        Game {
            index,
            draws
        }
    }

    fn draws_possible_with_return(&self, max_red: i32, max_green: i32, max_blue: i32) -> bool {
        self.draws.iter().all(|draw| {
            draw.red <= max_red && draw.green <= max_green && draw.blue <= max_blue
        })
    }

    fn min_full_draw(&self) -> Draw {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in &self.draws {
            min_red = max(min_red, draw.red);
            min_green = max(min_green, draw.green);
            min_blue = max(min_blue, draw.blue);
        }
        Draw {red: min_red, green: min_green, blue: min_blue}
    }
}

fn solve_part_1(games: &Vec<Game>) -> i32 {
    let mut result = 0;
    for game in games {
        if game.draws_possible_with_return(12, 13, 14) {
            result += game.index;
        }
    }
    result
}

fn solve_part_2(games: &Vec<Game>) -> i32 {
    let mut result = 0;
    for game in games {
        result += game.min_full_draw().get_power();
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<i32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut games = Vec::<Game>::new();
    for line in file_contents.lines() {
        games.push(Game::parse(line));
    }
    let result = match task_part {
        1 => solve_part_1(&games),
        2 => solve_part_2(&games),
        _ => -1
    };
    println!("{}", result);
}
