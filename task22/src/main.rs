use std::cmp::{min, max};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(Clone)]
struct ValueRange {
    min: usize,
    max: usize
}

impl ValueRange {
    fn new(min: usize, max: usize) -> Self {
        Self {
            min, max
        }
    }

    fn overlapping (&self, other: &Self) -> bool {
        (self.min >= other.min && self.min <= other.max) ||
        (self.max >= other.min && self.max <= other.max) ||
        (other.min >= self.min && other.min <= self.max) ||
        (other.max >= self.min && other.max <= self.max)
    }
}

#[derive(Clone)]
struct CoordinateRanges {
    x: ValueRange,
    y: ValueRange
}

impl CoordinateRanges {
    fn new(x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        Self {
            x: ValueRange::new(x_min, x_max),
            y: ValueRange::new(y_min, y_max)
        }
    }

    fn overlapping(&self, other: &Self) -> bool {
        self.x.overlapping(&other.x) && self.y.overlapping(&other.y)
    }
}

#[derive(Clone)]
struct BrickLayer {
    id: usize,
    height: usize,
    sides: CoordinateRanges,
    is_bottom_layer: bool
}

impl BrickLayer {
    fn new(id: usize, height: usize, x_min: usize, x_max: usize, y_min: usize, y_max: usize) -> Self {
        Self {
            id,
            height,
            sides: CoordinateRanges::new(x_min, x_max, y_min, y_max),
            is_bottom_layer: true
        }
    }
}

struct BrickStructure {
    brick_layers: BTreeMap<usize, Vec<BrickLayer>>, // layer height to brick layers occupying it
    layer_to_supporting: HashMap<usize, Vec<usize>>, // layer ID to vector of all layers supporting it
    layer_to_supported_by: HashMap<usize, Vec<usize>> // layer ID to vector of all layers supported by it
}

impl BrickStructure {
    fn new(input: &str) -> Self {
        let mut brick_layers = BTreeMap::new();
        for (id, line) in input.lines().enumerate() {
            let (start_coordinates, end_coordinates) = line.split_once('~').unwrap();
            let start_coordinates: Vec<_> = start_coordinates.split(',').collect();
            let x1 = start_coordinates[0].parse().unwrap();
            let y1 = start_coordinates[1].parse().unwrap();
            let z1 = start_coordinates[2].parse().unwrap();
            let end_coordinates: Vec<_> = end_coordinates.split(',').collect();
            let x2 = end_coordinates[0].parse().unwrap();
            let y2 = end_coordinates[1].parse().unwrap();
            let z2: usize = end_coordinates[2].parse().unwrap();
            brick_layers.entry(min(z1, z2)).or_insert(vec![]).push(BrickLayer::new(id, (z2 as i32 - z1 as i32).abs() as usize, min(x1, x2), max(x1, x2), min(y1, y2), max(y1, y2)));
        }
        Self {
            brick_layers,
            layer_to_supporting: HashMap::new(),
            layer_to_supported_by: HashMap::new()
        }
    }

    fn fall_down(&mut self) {
        let mut new_layers: BTreeMap<usize, Vec<BrickLayer>> = BTreeMap::new();
        for (height, layers) in &self.brick_layers {
            for layer in layers {
                let mut current_height = *height;
                loop {
                    if current_height == 1 {
                        self.layer_to_supporting.insert(layer.id, vec![]);
                        self.layer_to_supported_by.entry(layer.id).or_insert(vec![]);
                        break;
                    }
                    let mut supporting_found = false;
                    for lower_layer in new_layers.get(&(current_height - 1)).unwrap_or(&vec![]) {
                        if layer.sides.overlapping(&lower_layer.sides) {
                            supporting_found = true;
                            self.layer_to_supporting.entry(layer.id).or_insert(vec![]).push(lower_layer.id);
                            self.layer_to_supported_by.entry(layer.id).or_insert(vec![]);
                            self.layer_to_supported_by.entry(lower_layer.id).or_insert(vec![]).push(layer.id);
                        }
                    }
                    if supporting_found {
                        break;
                    }

                    current_height -= 1;
                }
                new_layers.entry(current_height).or_insert(vec![]).push(layer.clone());
                if layer.height > 0 {
                    let mut top_layer = layer.clone();
                    top_layer.is_bottom_layer = false;
                    new_layers.entry(current_height + top_layer.height).or_insert(vec![]).push(top_layer);
                }
            }
        }
        self.brick_layers = new_layers;
    }

    fn count_safe_to_disintegrate(&self) -> usize {
        let mut result = 0;
        for (_, supported) in &self.layer_to_supported_by {
            if supported.iter().all(|supported_id| self.layer_to_supporting[supported_id].len() > 1) {
                result += 1;
            }
        }
        result
    }

    fn count_falling(&self) -> usize {
        let mut result = 0;
        for (id, supported) in &self.layer_to_supported_by {
            let mut bricks_to_check = VecDeque::new();
            let mut falling_bricks = HashSet::new();
            for brick in supported {
                if self.layer_to_supporting[brick].len() == 1 {
                    result += 1;
                    bricks_to_check.push_back(brick);
                    falling_bricks.insert(brick);
                }
            }
            while let Some(falling_brick) = bricks_to_check.pop_front() {
                for brick in &self.layer_to_supported_by[falling_brick] {
                    if self.layer_to_supporting[brick].iter().all(|supporting_brick| falling_bricks.contains(&supporting_brick)) {
                        if falling_bricks.insert(brick) {
                            result += 1;
                            bricks_to_check.push_back(brick);
                        }
                    }
                }
            }
        }
        result
    }
}

fn solve_part_1(structure: &mut BrickStructure) -> usize {
    structure.fall_down();
    structure.count_safe_to_disintegrate()
}

fn solve_part_2(structure: &mut BrickStructure) -> usize {
    structure.fall_down();
    structure.count_falling()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut structure = BrickStructure::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&mut structure),
        2 => solve_part_2(&mut structure),
        _ => 0
    };
    println!("{}", result);
}
