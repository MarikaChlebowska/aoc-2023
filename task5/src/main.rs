use std::collections::BTreeMap;
use std::env;
use std::fs;

struct Translation {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32
}

struct Translator {
    translations: BTreeMap<u32, Translation>,
    reverse_translations: BTreeMap<u32, Translation>
}

impl Translator {
    fn new(input: &str) -> Self {
        let mut translations = BTreeMap::new();
        let mut reverse_translations = BTreeMap::new();
        for line in input.lines().skip(1) {
            let fields: Vec<&str> = line.split(' ').collect();
            let destination_range_start = fields[0].parse::<u32>().unwrap();
            let source_range_start = fields[1].parse::<u32>().unwrap();
            let range_length = fields[2].parse::<u32>().unwrap();
            translations.insert(source_range_start, Translation{destination_range_start, source_range_start, range_length});
            reverse_translations.insert(destination_range_start, Translation{destination_range_start, source_range_start, range_length});
        }
        Self {
            translations,
            reverse_translations
        }
    }

    fn translate(&self, source: u32) -> u32 {
        let mut range = self.translations.range(..=source);
        match range.next_back() {
            None => source,
            Some((_, translator)) => {
                let start_distance = source - translator.source_range_start;
                if start_distance <= translator.range_length {
                    translator.destination_range_start + start_distance
                }
                else {
                    source
                }
            }
        }
    }

    fn reverse_translate(&self, destination: u32) -> u32 {
        let mut range = self.reverse_translations.range(..=destination);
        match range.next_back() {
            None => destination,
            Some((_, translator)) => {
                let start_distance = destination - translator.destination_range_start;
                if start_distance <= translator.range_length {
                    translator.source_range_start + start_distance
                }
                else {
                    destination
                }
            }
        }
    }
}

struct TranslatorChain {
    translators: Vec<Translator>
}

impl TranslatorChain {
    fn new(input: &str) -> Self {
        let mut translators = vec![];
        for translator_input in input.split("\n\n") {
            translators.push(Translator::new(translator_input));
        }
        Self {translators}
    }

    fn translate(&self, source: u32) -> u32 {
        let mut destination  = source;
        for translator in &self.translators {
            destination = translator.translate(destination);
        }
        destination
    }

    fn reverse_translate(&self, destination: u32) -> u32 {
        let mut source = destination;
        for translator in self.translators.iter().rev() {
            source = translator.reverse_translate(source);
        }
        source
    }
}

struct SeedRanges {
    ranges: BTreeMap<u32, u32>
}

impl SeedRanges {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new()
        }
    }

    fn is_in_ranges(&self, seed: u32) -> bool {
        let mut range = self.ranges.range(..=seed);
        match range.next_back() {
            None => false,
            Some((range_start, range_length)) => {
                let start_distance = seed - range_start;
                if start_distance <= *range_length {
                    true
                }
                else {
                    false
                }
            }
        }
    }
}



fn get_seeds(input: &str) -> Vec<u32> {
    let (_, seed_list) = input.split_once(": ").unwrap();
    let mut seeds = vec![];
    for seed in seed_list.split(' ') {
        seeds.push(seed.parse::<u32>().unwrap())
    }
    seeds
}

fn parse_input(input: &str) -> (Vec<u32>, TranslatorChain) {
    let (seeds_input, translation_input) = input.split_once("\n\n").unwrap();
    (get_seeds(seeds_input), TranslatorChain::new(translation_input))
}

fn solve_part_1(seeds: &Vec<u32>, chain: &TranslatorChain) -> i32 {
    let mut destinations = vec![];
    for seed in seeds {
        destinations.push(chain.translate(*seed));
    }
    *destinations.iter().min().unwrap() as i32
}

fn solve_part_2(seeds: &Vec<u32>, chain: &TranslatorChain) -> i32 {
    let mut seed_ranges = SeedRanges::new();
    for chunk in seeds.chunks(2) {
        seed_ranges.ranges.insert(chunk[0], chunk[1]);
    }
    // This is a brute force solution, it would be better to combine all translations into one
    for destination in 0.. {
        let source = chain.reverse_translate(destination);
        if seed_ranges.is_in_ranges(source) {
            return destination as i32;
        }
    }
    -1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u32>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let (seeds, chain) = parse_input(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&seeds, &chain),
        2 => solve_part_2(&seeds, &chain),
        _ => -1
    };
    println!("{}", result);
}
