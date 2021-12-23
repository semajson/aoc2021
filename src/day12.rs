use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::num;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Node(String);
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Graph(HashMap<Node, Vec<Node>>);
impl Graph {
    pub fn new(input_lines: Vec<&String>) -> Result<Graph, num::ParseIntError> {
        let input_lines = input_lines.to_owned();

        let mut graph: HashMap<Node, Vec<Node>> = HashMap::new();

        for line in input_lines.iter() {
            let nodes = line
                .split('-')
                .map(|x| Node(x.to_string()))
                .collect::<Vec<Node>>();

            if let Some(reachable_nodes) = graph.get_mut(&nodes[0]) {
                reachable_nodes.push(nodes[1].clone());
            } else {
                graph.insert(nodes[0].clone(), vec![nodes[1].clone()]);
            }

            if let Some(reachable_nodes) = graph.get_mut(&nodes[1]) {
                reachable_nodes.push(nodes[0].clone());
            } else {
                graph.insert(nodes[1].clone(), vec![nodes[0].clone()]);
            }
        }

        Ok(Graph(graph))
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Graph, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    Graph::new(input_lines)
}

pub fn part_1(grid: &Graph) -> i64 {
    println!("test");
    0
}

pub fn part_2(grid: &Graph) -> i64 {
    0
}

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&parsed_data) as u64, part_2(&parsed_data) as u64)
}
