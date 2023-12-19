use std::collections::HashMap;
use std::cmp::{min, max};
use std::env;
use std::fs;

struct Toy {
    x: i64,
    m: i64,
    a: i64,
    s: i64
}

impl Toy {
    fn new(input: &str) -> Self {
        let input = &input[1..input.len() - 1];
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;
        for statistic in input.split(',') {
            let (name, ammount) = statistic.split_once('=').unwrap();
            match name.chars().next().unwrap() {
                'x' => x = ammount.parse().unwrap(),
                'm' => m = ammount.parse().unwrap(),
                'a' => a = ammount.parse().unwrap(),
                's' => s = ammount.parse().unwrap(),
                _ => panic!("unknown statistic")
            }
        }
        Self {x, m, a, s}
    }

    fn get_statistic(&self, name: char) -> i64 {
        match name {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("unknown statistic queried")
        }
    }

    fn get_value(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

enum Condition {
    LT(char, i64),
    GT(char, i64),
    None
}

impl Condition {
    fn new(input: &str) -> Self {
        if input.is_empty() {
            Condition::None
        } else if input.contains('<') {
            let (name, value) = input.split_once('<').unwrap();
            Condition::LT(name.chars().next().unwrap(), value.parse().unwrap())
        } else if input.contains('>') {
            let (name, value) = input.split_once('>').unwrap();
            Condition::GT(name.chars().next().unwrap(), value.parse().unwrap())
        } else {
            panic!("Unknown condition")
        }
    }

    fn apply(&self, toy: &Toy) -> bool {
        match self {
            Condition::LT(statistic, ammount) => toy.get_statistic(*statistic) < *ammount,
            Condition::GT(statistic, ammount) => toy.get_statistic(*statistic) > *ammount,
            Condition::None => true
        }
    }
}

#[derive(Clone)]
struct Constrain {
    min_value: i64,
    max_value: i64
}

impl Constrain {
    fn new() -> Self {
        Self {
            min_value: 1,
            max_value: 4000
        }
    }

    fn possible(&self) -> bool {
        self.min_value <= self.min_value
    }

    fn count_possibilities(&self) -> i64 {
        self.max_value - self.min_value + 1
    }
}

#[derive(Clone)]
struct ConstrainSet {
    x: Constrain,
    m: Constrain,
    a: Constrain,
    s: Constrain,
}

impl ConstrainSet {
    fn new() -> Self {
        Self {
            x: Constrain::new(),
            m: Constrain::new(),
            a: Constrain::new(),
            s: Constrain::new()
        }
    }

    fn get_constrain(&mut self, name: char) -> &mut Constrain {
        match name {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!("unknown statistic queried")
        }
    }

    fn apply_condition(&mut self, condition: &Condition) {
        match condition {
            Condition::LT(name, ammount) => {
                let constrain = self.get_constrain(*name);
                constrain.max_value = min(constrain.max_value, ammount - 1);
            },
            Condition::GT(name, ammount) => {
                let constrain = self.get_constrain(*name);
                constrain.min_value = max(constrain.min_value, ammount + 1);
            },
            Condition::None => {}
        }
    }

    fn apply_condition_negation(&mut self, condition: &Condition) {
        match condition {
            Condition::LT(name, ammount) => {
                let constrain = self.get_constrain(*name);
                constrain.min_value = max(constrain.min_value, *ammount);
            },
            Condition::GT(name, ammount) => {
                let constrain = self.get_constrain(*name);
                constrain.max_value = min(constrain.max_value, *ammount);
            },
            Condition::None => {}
        }
    }

    fn count_possibilities(&self) -> i64 {
        self.x.count_possibilities() * self.m.count_possibilities() * self.a.count_possibilities() * self.s.count_possibilities()
    }

    fn possible(&self) -> bool {
        self.x.possible() && self.m.possible() && self.a.possible() && self.s.possible()
    }
}

struct WorkflowStep {
    condition: Condition,
    target: String
}

impl WorkflowStep {
    fn new(input: &str) -> Self {
        let condition_str;
        let target_str;
        if input.contains(':') {
            (condition_str, target_str) = input.split_once(':').unwrap();
        } else {
            condition_str = "";
            target_str = input;
        }
        let condition = Condition::new(condition_str);
        let target = target_str.to_string();

        Self {
            condition,
            target
        }
    }

    fn apply(&self, toy: &Toy) -> Option<String> {
        if self.condition.apply(toy) {
            Some(self.target.clone())
        } else {
            None
        }
    }
}

struct Workflow {
    steps: Vec<WorkflowStep>
}

impl Workflow {
    fn new(input: &str) -> Self {
        let mut steps = vec![];
        for step in input.split(',') {
            steps.push(WorkflowStep::new(step));
        }
        Self{steps}
    }

    fn apply(&self, toy: &Toy) -> String {
        for step in &self.steps {
            if let Some(next_workflow) = step.apply(toy) {
                return next_workflow;
            }
        }
        panic!("No next workflow determined");
    }

    fn back_track(&self, target: &String, constrains_so_far: &ConstrainSet) -> Vec<ConstrainSet> {
        let mut possible_constrains: Vec<ConstrainSet> = vec![];
        for step in self.steps.iter().rev() {
            for constrain_set in &mut possible_constrains {
                constrain_set.apply_condition_negation(&step.condition);
            }
            if &step.target == target {
                let mut new_constrain  = constrains_so_far.clone();
                new_constrain.apply_condition(&step.condition);
                possible_constrains.push(new_constrain);
            }
        }
        possible_constrains.iter().filter(|constrain| constrain.possible()).map(|x| x.clone()).collect()
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Toy>) {
    let mut workflow_map = HashMap::new();
    let mut toys = vec![];
    let (workflows_input, toys_input) = input.split_once("\n\n").unwrap();
    for workflow_input in workflows_input.lines() {
        let (name, workflow) = workflow_input.split_once('{').unwrap();
        workflow_map.insert(name.to_string(), Workflow::new(&workflow[..workflow.len() - 1]));
    }
    for toy_input in toys_input.lines() {
        toys.push(Toy::new(toy_input));
    }
    (workflow_map, toys)
}

fn insert_into_multimap(multimap: &mut HashMap<String, Vec<ConstrainSet>>, key: &String, constrain_sets: &mut Vec<ConstrainSet>) {
    if multimap.contains_key(key) {
        multimap.get_mut(key).unwrap().append(constrain_sets);
    } else if !constrain_sets.is_empty() {
        multimap.insert(key.clone(), constrain_sets.clone());
    }
}

fn solve_part_1(workflows: &HashMap<String, Workflow>, toys: &Vec<Toy>) -> i64 {
    let mut result = 0;
    for toy in toys {
        let mut current_workflow = "in".to_string();
        loop {
            if current_workflow == "A" {
                result += toy.get_value();
                break;
            } else if current_workflow == "R" {
                break;
            }
            current_workflow = workflows[&current_workflow].apply(toy);
        }
    }
    result
}

fn solve_part_2(workflows: &HashMap<String, Workflow>) -> i64 {
    let mut result = 0;
    let mut target_condition_map: HashMap<String, Vec<ConstrainSet>> = HashMap::new();
    for (source, workflow) in workflows {
        let mut constriction_sets = workflow.back_track(&"A".to_string(), &ConstrainSet::new());
        insert_into_multimap(&mut target_condition_map, source, &mut constriction_sets);
    }
    while !target_condition_map.is_empty() {
        let target = target_condition_map.keys().next().unwrap().clone();
        let constrain_sets = target_condition_map.remove(&target).unwrap();
        if target == "in".to_string() {
            for constrain_set in &constrain_sets {
                result += constrain_set.count_possibilities();
            }
        } else {
            for (source, workflow) in workflows {
                for constrain_set in &constrain_sets {
                    let mut constriction_sets = workflow.back_track(&target, constrain_set);
                    insert_into_multimap(&mut target_condition_map, source, &mut constriction_sets);
                }
            }
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let (workflows, toys) = parse_input(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&workflows, &toys),
        2 => solve_part_2(&workflows),
        _ => 0
    };
    println!("{}", result);
}
