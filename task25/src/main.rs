use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

type GraphEdge = (String, String);

fn get_graph_edge(node1: &str, node2: &str) -> GraphEdge {
    if node2 > node1 {
        (node1.to_string(), node2.to_string())
    } else {
        (node2.to_string(), node1.to_string())
    }
}

fn get_edges_from_path(path: &Vec<String>) -> Vec<GraphEdge> {
    path[..].windows(2).map(|elements| get_graph_edge(&elements[0], &elements[1])).collect()
}

#[derive(Clone, Debug)]
struct Graph {
    connections: HashMap<String, HashSet<String>>
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut connections = HashMap::new();
        for line in input.lines() {
            let (source, destinations) = line.split_once(": ").unwrap();
            for destination in destinations.split(' ') {
                connections.entry(source.to_string()).or_insert(HashSet::new()).insert(destination.to_string());
                connections.entry(destination.to_string()).or_insert(HashSet::new()).insert(source.to_string());
            }
        }
        Self {connections}
    }

    fn remove_edge(&mut self, edge: &GraphEdge) {
        self.connections.get_mut(&edge.0).unwrap().remove(&edge.1);
        self.connections.get_mut(&edge.1).unwrap().remove(&edge.0);
    }

    fn find_path_without_edges(&self, start: &str, end: &str, forbidden_edges: &HashSet<GraphEdge>) -> Option<Vec<GraphEdge>> {
        if !self.connections.contains_key(start) || !self.connections.contains_key(end) {
            return None;
        } else if start == end {
            return Some(vec![])
        }
        let mut paths_to_search = VecDeque::new();
        let mut visited_nodes = HashSet::new();
        visited_nodes.insert(start.to_string());
        for next_step in &self.connections[start] {
            if !forbidden_edges.contains(&get_graph_edge(start, next_step)){
                paths_to_search.push_back(vec![start.to_string(), next_step.clone()]);
            }
        }
        while let Some(current_path) = paths_to_search.pop_front() {
            let current_node = current_path.last().unwrap();
            if current_node == end {
                return Some(get_edges_from_path(&current_path));
            }
            for next_step in &self.connections[current_node] {
                if !forbidden_edges.contains(&get_graph_edge(current_node, next_step)) &&
                    !visited_nodes.contains(next_step) {
                        let mut new_path = current_path.clone();
                        new_path.push(next_step.clone());
                        if next_step == end {
                            return Some(get_edges_from_path(&new_path));
                        }
                        paths_to_search.push_back(new_path);
                        visited_nodes.insert(next_step.clone());
                }
            }
        }
        None
    }

    fn get_unique_paths(&self, start: &str, end: &str) -> Vec<Vec<GraphEdge>> {
        if start == end {
            return vec![];
        }
        let mut result = vec![];
        let mut visited_edges = HashSet::new();
        while let Some(edges) = self.find_path_without_edges(start, end, &visited_edges) {
            visited_edges.extend(edges.iter().map(|x| x.clone()));
            result.push(edges);
        }
        result
    }

    fn remove_disconnected(&mut self, node: &str) {
        let nodes: Vec<_> = self.connections.keys().map(|x| x.clone()).collect();
        for target in nodes {
            if let None = self.find_path_without_edges(node, &target, &HashSet::new()) {
                self.connections.remove(&target);
            }
        }
    }

    fn split_into_two(&self, node1: &str, node2: &str) -> (Self, Self) {
        let mut new_graph = self.clone();
        'a: loop {
            let unique_paths = new_graph.get_unique_paths(node1, node2);
            if unique_paths.len() == 0 {
                break;
            }
            for edge in &unique_paths[0] {
                let mut current_graph = new_graph.clone();
                current_graph.remove_edge(&edge);
                if current_graph.get_unique_paths(node1, node2).len() < unique_paths.len() {
                    new_graph = current_graph;
                    continue 'a;
                }
            }
            panic!("Can't split paths!");
        }
        let mut graph1 = new_graph.clone();
        graph1.remove_disconnected(node1);
        let mut graph2 = new_graph.clone();
        graph2.remove_disconnected(node2);
        (graph1, graph2)
    }
}

fn solve_part_1(graph: &Graph) -> usize {
    
    let node1 = graph.connections.keys().next().unwrap();
    let mut node2 = node1;
    for node in graph.connections.keys() {
        let unique_path_number = graph.get_unique_paths(node1, node).len();
        if unique_path_number <= 3 && unique_path_number > 0 {
            node2 = node;
        }
    }
    let (graph1, graph2) = graph.split_into_two(node1, node2);
    graph1.connections.len() * graph2.connections.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let task_part = args[1].parse::<u64>().unwrap();
    let file_path = &args[2];
    let file_contents = fs::read_to_string(file_path).unwrap();
    let graph = Graph::new(&file_contents);
    let result = match task_part {
        1 => solve_part_1(&graph),
        _ => 0
    };
    println!("{}", result);
}
