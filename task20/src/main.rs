use std::collections::{VecDeque, HashMap, HashSet};
use std::env;
use std::fs;

#[derive(PartialEq, Eq, Clone, Debug, Hash, PartialOrd, Ord)]
enum SignalType {
    High,
    Low
}

impl SignalType {
    fn opposite(&self) -> Self {
        match self {
            SignalType::High => SignalType::Low,
            SignalType::Low => SignalType::High
        }
    }
}

struct Signal {
    signal_type: SignalType,
    source: String,
    destination: String
}

impl Signal {
    fn new(signal_type: SignalType, source: &str, destination: &str) -> Self {
        Self {
            signal_type,
            source: source.to_string(),
            destination: destination.to_string()
        }
    }
}

trait Module {
    fn add_input(&mut self, _input_name: &str) {

    }

    fn receive_signal(&mut self, signal: &Signal) -> VecDeque<Signal>;

    fn command_received(&self) -> bool {
        false
    }
}

#[derive(Clone)]
struct FlipFlop {
    name: String,
    state: SignalType, // SignalType::Low = state OFF
    outputs: Vec<String>
}

impl FlipFlop {
    fn new(name: &str, outputs: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            state: SignalType::Low,
            outputs
        }
    }
}

impl Module for FlipFlop {
    fn receive_signal(&mut self, signal: &Signal) -> VecDeque<Signal> {
        if let SignalType::Low = signal.signal_type {
            self.state = self.state.opposite();
            self.outputs.iter().map(|output| Signal::new(self.state.clone(), &self.name, output)).collect()
        } else {
            VecDeque::new()
        }
    }
}

#[derive(Clone)]
struct Conjunction {
    name: String,
    inputs_in_memory: HashMap<String, SignalType>,
    outputs: Vec<String>,
    is_inverter: bool
}

impl Conjunction {
    fn new(name: &str, outputs: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            inputs_in_memory: HashMap::new(),
            outputs,
            is_inverter: true
        }
    }
}

impl Module for Conjunction {
    fn add_input(&mut self, input_name: &str) {
        self.inputs_in_memory.insert(input_name.to_string(), SignalType::Low);
        if self.inputs_in_memory.len() > 1 {
            self.is_inverter = false;
        }
    }

    fn receive_signal(&mut self, signal: &Signal) -> VecDeque<Signal> {
        if self.is_inverter {
            return self.outputs.iter().map(|output| Signal::new(signal.signal_type.opposite(), &self.name, output)).collect();
        }
        *self.inputs_in_memory.get_mut(&signal.source).unwrap() = signal.signal_type.clone();
        let output_signal = if self.inputs_in_memory.values().all(|signal_type| signal_type == &SignalType::High) {
            SignalType::Low
        } else {
            SignalType::High
        };
        self.outputs.iter().map(|output| Signal::new(output_signal.clone(), &self.name, output)).collect()
    }
}

#[derive(Clone)]
struct Broadcast {
    name: String,
    outputs: Vec<String>
}

impl Broadcast {
    fn new(name: &str, outputs: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            outputs
        }
    }
}

impl Module for Broadcast {
    fn receive_signal(&mut self, signal: &Signal) -> VecDeque<Signal> {
        self.outputs.iter().map(|output| Signal::new(signal.signal_type.clone(), &self.name, output)).collect()
    }
}

#[derive(Clone)]
struct Receiver {
    low_state_reached: bool,
}

impl Receiver {
    fn new() -> Self {
        Self { low_state_reached: false }
    }
}

impl Module for Receiver {
    fn receive_signal(&mut self, signal: &Signal) -> VecDeque<Signal> {
        if let SignalType::Low = signal.signal_type {
            self.low_state_reached = true;
        }
        VecDeque::new()
    }

    fn command_received(&self) -> bool {
        self.low_state_reached
    }
}

fn build_module(input: &str) -> (String, Box<dyn Module>, Vec<String>) { // returns (name, module, outputs)
    let (module, outputs_str) = input.split_once(" -> ").unwrap();
    let outputs: Vec<String> = outputs_str.split(", ").map(|string| string.to_string()).collect();
    let first_character = module.chars().next().unwrap();
    match first_character {
        '%' => (module[1..].to_string(), Box::new( FlipFlop::new(&module[1..], outputs.clone())), outputs),
        '&' => (module[1..].to_string(),Box::new(Conjunction::new(&module[1..], outputs.clone())), outputs),
        'b' => (module.to_string(),Box::new(Broadcast::new(module, outputs.clone())), outputs),
        _ => panic!("Unknown module type {}", module)
    }
}

struct Machine {
    modules: HashMap<String, Box<dyn Module>>,
    output_to_inputs: HashMap<String, Vec<String>>
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut modules = HashMap::new();
        let mut output_to_inputs = HashMap::new();
        for line in input.lines() {
            let (name, module, outputs) = build_module(line);
            modules.insert(name.clone(), module);
            for output in outputs {
                output_to_inputs.entry(output).or_insert(vec![]).push(name.clone());
            }
        }
        for (output, inputs) in &output_to_inputs {
            for input in inputs {
                if let Some(module) = modules.get_mut(output) {
                    module.add_input(&input);
                }
            }
        }
        Self { modules, output_to_inputs }
    }

        fn push_button(&mut self) -> (usize, usize) { // (low sent, high sent)
            let mut signals = VecDeque::from(vec![Signal::new(SignalType::Low, "button", "broadcaster")]);
            let mut low_sent = 0;
            let mut high_sent = 0;
            while let Some(signal) = signals.pop_front() {
                match signal.signal_type {
                    SignalType::Low => low_sent += 1,
                    SignalType::High => high_sent += 1,
                }
                if let Some(module) = self.modules.get_mut(&signal.destination) {
                    signals.append(&mut module.receive_signal(&signal));
                }
            }
            (low_sent, high_sent)
        }

        fn remove_irrelevant_modules(&mut self, target_module: &str) {
            let mut modules_to_check = VecDeque::new();
            let mut modules_to_keep = HashSet::new();
            modules_to_check.push_back(target_module.to_string());
            while let Some(module_name) = modules_to_check.pop_front() {
                if !modules_to_keep.contains(&module_name) {
                    modules_to_keep.insert(module_name.clone());
                    for module in self.output_to_inputs.get(&module_name).unwrap_or(&vec![]).iter() {
                        modules_to_check.push_back(module.clone());
                    }
                }
            }
            self.modules.insert(target_module.to_string(), Box::new(Receiver::new()));
            self.modules.retain(|name, _| modules_to_keep.contains(name));
        }
    }

fn solve_part_1(machine: &mut Machine) -> usize {
    let mut low_accumulator = 0;
    let mut high_accumulator = 0;
    for _ in 0..1000 {
        let (low_sent, high_sent) = machine.push_button();
        low_accumulator += low_sent;
        high_accumulator += high_sent;
    }
    low_accumulator * high_accumulator
}

fn solve_part_2(file_contents: &str) -> usize {
    // Partially hard-coded solution, after analysis of the system when the 4 targets are in low state at the same time rx is also in low state
    // Each of them reaches this state at the last iteration of their cycle
    let mut goal_iterations = vec![];
    for current_target in vec!["mr", "kk", "gl", "bb"] {
        let mut machine = Machine::new(&file_contents);
        let mut i: usize = 1;
        machine.remove_irrelevant_modules(current_target);
        loop {
            machine.push_button();
            if machine.modules[current_target].command_received() {
                goal_iterations.push(i);
                break;
            }
            i += 1;
        }
    }
    // the solution is LCM of the numbers, they're all prime numbers
    goal_iterations.iter().fold(1, |x, y| x * y)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let mut machine = Machine::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&mut machine),
        2 => solve_part_2(&file_contents),
        _ => 0
    };
    println!("{}", result);
}
