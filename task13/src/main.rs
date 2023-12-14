use std::env;
use std::fs;

type AshMap = Vec<Vec<char>>;

fn parse_map(map_input: &str) -> AshMap {
    let mut result = AshMap::new();
    for line in map_input.lines() {
        result.push(line.chars().collect());
    }
    result
}

fn parse_input(input: &str) -> Vec<AshMap> {
    let mut result = vec![];
    for ash_map in input.split("\n\n") {
        result.push(parse_map(ash_map));
    }
    result
}

fn check_mirroring_horizontal(ash_map: &AshMap, index: usize) -> bool {
    let mut index_low = index;
    let mut index_high = index + 1;
    while index_low > 0 && index_high < ash_map.len() - 1 {
        if ash_map[index_low] != ash_map[index_high] {
            return false;
        }
        index_low -= 1;
        index_high += 1;
    }
    
    if ash_map[index_low] != ash_map[index_high] {
        false
    }
    else {
        true
    }
}

fn check_mirroring_vertical(ash_map: &AshMap, index: usize) -> bool {
    let mut index_low = index;
    let mut index_high = index + 1;
    while index_low > 0 && index_high < ash_map[0].len() - 1 {
        if ash_map.iter().map(|row| row[index_low]).ne(ash_map.iter().map(|row| row[index_high])) {
            return false;
        }
        index_low -= 1;
        index_high += 1;
    }
    
    if ash_map.iter().map(|row| row[index_low]).ne(ash_map.iter().map(|row| row[index_high])) {
        false
    }
    else {
        true
    }
}

fn check_mirroring_horizontal_with_smudge(ash_map: &AshMap, index: usize) -> bool {
    let mut index_low = index;
    let mut index_high = index + 1;
    let mut smudge_found = false;
    let row_length = ash_map[0].len();
    while index_low > 0 && index_high < ash_map.len() - 1 {
        if !smudge_found {
            let matches = ash_map[index_low].iter().zip(ash_map[index_high].iter()).filter(|x| x.0 == x.1).count();
            if matches == row_length - 1 {
                smudge_found = true;
            }
            else if matches < row_length - 1 {
                return false;
            }
        }
        else if ash_map[index_low] != ash_map[index_high] {
            return false;
        }
        index_low -= 1;
        index_high += 1;
    }
    
    
    if !smudge_found {
        let matches = ash_map[index_low].iter().zip(ash_map[index_high].iter()).filter(|x| x.0 == x.1).count();
        if matches == row_length - 1 {
            true
        }
        else{
            false
        }
    }
    else {
        ash_map[index_low] == ash_map[index_high]
    }
}

fn check_mirroring_vertical_with_smudge(ash_map: &AshMap, index: usize) -> bool {
    let mut index_low = index;
    let mut index_high = index + 1;
    let mut smudge_found = false;
    let column_height = ash_map.len();
    while index_low > 0 && index_high < ash_map[0].len() - 1 {
        if !smudge_found {
            let matches = ash_map.iter().map(|row| row[index_low]).zip(ash_map.iter().map(|row| row[index_high])).filter(|x| x.0 == x.1).count();
            if matches == column_height - 1 {
                smudge_found = true;
            }
            else if matches < column_height - 1 {
                return false;
            }
        }
        else if ash_map.iter().map(|row| row[index_low]).ne(ash_map.iter().map(|row| row[index_high])) {
            return false;
        }
        index_low -= 1;
        index_high += 1;
    }
    
    if !smudge_found {
        let matches = ash_map.iter().map(|row| row[index_low]).zip(ash_map.iter().map(|row| row[index_high])).filter(|x| x.0 == x.1).count();
        if matches == column_height - 1 {
            true
        }
        else {
            false
        }
    }
    else {
        ash_map.iter().map(|row| row[index_low]).eq(ash_map.iter().map(|row| row[index_high]))
    }
}

fn find_horizontal_separator(ash_map: &AshMap) -> Option<usize> {
    for i in 0..ash_map.len() - 1 {
        if check_mirroring_horizontal(ash_map, i) {
            return Some(i);
        }
    }
    None
}

fn find_vertical_separator(ash_map: &AshMap) -> Option<usize> {
    for i in 0..ash_map[0].len() - 1 {
        if check_mirroring_vertical(ash_map, i) {
            return Some(i);
        }
    }
    None
}

fn find_horizontal_separator_with_smudge(ash_map: &AshMap) -> Option<usize> {
    for i in 0..ash_map.len() - 1 {
        if check_mirroring_horizontal_with_smudge(ash_map, i) {
            return Some(i);
        }
    }
    None
}

fn find_vertical_separator_with_smudge(ash_map: &AshMap) -> Option<usize> {
    for i in 0..ash_map[0].len() - 1 {
        if check_mirroring_vertical_with_smudge(ash_map, i) {
            return Some(i);
        }
    }
    None
}

fn solve_part_1(ash_maps: &Vec<AshMap>) -> usize {
    let mut columns_to_left = 0;
    let mut rows_above = 0;
    for ash_map in ash_maps {
        if let Some(horizontal_separator) = find_horizontal_separator(ash_map) {
            rows_above += horizontal_separator + 1;
        }
        else if let Some(vertical_separator) = find_vertical_separator(ash_map) {
            columns_to_left += vertical_separator + 1;
        }
        else {
            panic!("No mirroring found");
        }
    }

    columns_to_left + 100 * rows_above
}

fn solve_part_2(ash_maps: &Vec<AshMap>) -> usize {
    let mut columns_to_left = 0;
    let mut rows_above = 0;
    for ash_map in ash_maps {
        if let Some(horizontal_separator) = find_horizontal_separator_with_smudge(ash_map) {
            rows_above += horizontal_separator + 1;
        }
        else if let Some(vertical_separator) = find_vertical_separator_with_smudge(ash_map) {
            columns_to_left += vertical_separator + 1;
        }
        else {
            panic!("No mirroring found");
        }
    }

    columns_to_left + 100 * rows_above
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let ash_maps = parse_input(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&ash_maps),
        2 => solve_part_2(&ash_maps),
        _ => 0
    };
    println!("{}", result);
}

// 2468777454
