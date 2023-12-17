use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Clone, PartialEq, Eq, Hash)]
struct SpringRow {
    springs: Vec<char>,
    damaged_groups: Vec<usize>
}

fn is_group_possible(springs: &Vec<char>, group: usize) -> bool {
    for i in 0..group {
        if springs[i] == '.' {
            return false;
        }
    }
    springs.len() == group || springs [group] != '#'
}

impl SpringRow {
    fn new(springs: &[char], damaged_groups: &[usize]) -> Self {
        Self {
            springs: springs.into(),
            damaged_groups: damaged_groups.into()
        }
    }

    fn from_input(input: &str) -> Self {
        let (spring_string, damaged_groups_string) = input.split_once(' ').unwrap();
        let springs: Vec<_> = spring_string.chars().collect();
        let damaged_groups: Vec<_> = damaged_groups_string.split(',').map(|x| x.parse().unwrap()).collect();
        Self {
            springs,
            damaged_groups
        }
    }

    fn unfold(&mut self, times: usize) {
        let initial_springs = self.springs.clone();
        for _ in 1..times {
            self.springs.push('?');
            self.springs.extend(initial_springs.iter());
        }
        self.damaged_groups = self.damaged_groups.iter().cycle().take(self.damaged_groups.len() * times).map(|x| *x).collect();
    }

    fn count_possible_combinations(&self, lookup_table: &mut HashMap<Self, u64>) -> u64 {
        if lookup_table.contains_key(self) {
            return lookup_table[self];
        }
        let mut combinations = 0;
        let mut damaged_found = false;
        if self.damaged_groups.len() == 0 {
            if self.springs.contains(&'#') {
                lookup_table.insert(self.clone(), 0);
                return 0;
            }
            else {
                lookup_table.insert(self.clone(), 1);
                return 1;
            }
        }
        let first_group = self.damaged_groups[0];
        let required_length = (self.damaged_groups.iter().sum::<usize>() + self.damaged_groups.len() - 1) as i32;
        let mut current_length = self.springs.len() as i32;
        let mut i = 0;
        while current_length >= required_length {
            match self.springs[i] {
                '#' => {
                    damaged_found = true;
                    if is_group_possible(&self.springs[i..].into(), first_group) {
                            combinations += if self.damaged_groups.len() > 1 {
                                SpringRow::new(&self.springs[i + first_group + 1..], &self.damaged_groups[1..]).count_possible_combinations(lookup_table)
                        } else if current_length as usize == first_group || current_length as usize == first_group + 1{
                            1
                        } else {
                            SpringRow::new(self.springs[i + first_group + 1..].into(), &vec![]).count_possible_combinations(lookup_table)
                        }
                    }
                },
                '?' => {
                    if is_group_possible(&self.springs[i..].into(), first_group) {
                            combinations += if self.damaged_groups.len() > 1 {
                                SpringRow::new(&self.springs[i + first_group + 1..], &self.damaged_groups[1..]).count_possible_combinations(lookup_table)
                        } else if current_length as usize == first_group || current_length as usize == first_group + 1{
                            1
                        } else {
                            SpringRow::new(self.springs[i + first_group + 1..].into(), &vec![]).count_possible_combinations(lookup_table)
                        }
                    }
                },
                _ => {}
            }
            if damaged_found {
                break;
            }
            i += 1;
            current_length -= 1;
        }
        lookup_table.insert(self.clone(), combinations);
        combinations
    }
}

fn solve_part_1(spring_rows: &Vec<SpringRow>) -> u64 {
    let mut result = 0;
    let mut lookup_table = HashMap::<SpringRow, u64>::new();
    for spring_row in spring_rows {
        result += spring_row.count_possible_combinations(&mut lookup_table);
    }
    result
}

fn solve_part_2(spring_rows: &mut Vec<SpringRow>) -> u64 {
    let mut result = 0;
    let mut lookup_table = HashMap::<SpringRow, u64>::new();
    for spring_row in spring_rows {
        spring_row.unfold(5);
        result += spring_row.count_possible_combinations(&mut lookup_table);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut spring_rows = vec![];
    for line in file_contents.lines() {
        spring_rows.push(SpringRow::from_input(line));
    }
    let result = match task_part {
        1 => solve_part_1(&spring_rows),
        2 => solve_part_2(&mut spring_rows),
        _ => 0
    };
    println!("{}", result);
}
