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
impl Node {
    fn is_start(&self) -> bool {
        self.0 == "start"
    }

    fn is_end(&self) -> bool {
        self.0 == "end".to_string()
    }

    fn is_large_cave(&self) -> bool {
        self.0.to_uppercase() == self.0
    }

    fn is_small_cave(&self) -> bool {
        (!self.is_large_cave()) && (!self.is_end()) && (!self.is_start())
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

    pub fn find_num_paths_start_end(&self) -> i64 {
        let start_node = self.get_start().unwrap();

        let mut completed_paths = vec![];

        let mut paths = vec![vec![start_node]];

        while paths.len() > 0 {
            let mut new_paths: Vec<Vec<Node>> = vec![];

            for path in paths.iter() {
                let reachable_nodes = self.0.get(path.last().unwrap()).unwrap();

                let reachable_nodes = reachable_nodes
                    .into_iter()
                    .filter(|node| node.is_large_cave() || !path.contains(node))
                    .map(|x| x.clone()) // don't like this
                    .collect::<Vec<Node>>();

                if reachable_nodes.len() == 0 {
                    // Dead path
                    continue;
                }

                for node in reachable_nodes.iter() {
                    let mut new_path = path.clone();
                    new_path.push(node.clone());
                    if node.is_end() {
                        completed_paths.push(new_path);
                    } else {
                        new_paths.push(new_path);
                    }
                }
            }
            paths = new_paths;
        }

        completed_paths.len() as i64
    }

    pub fn find_num_paths_start_end_part2(&self, allow_one_small_cave_twice: bool) -> i64 {
        let start_node = self.get_start().unwrap();

        let mut completed_paths = vec![];
        let mut paths = vec![vec![start_node]];

        // let mut paths_with_small_cave_twice = HashSet::new();

        while paths.len() > 0 {
            let mut new_paths: Vec<Vec<Node>> = vec![];

            for path in paths.iter() {
                let reachable_nodes = self.0.get(path.last().unwrap()).unwrap();

                let reachable_nodes = reachable_nodes
                    .into_iter()
                    .filter(|node| {
                        node.is_large_cave()
                            || !path.contains(node)
                            || (allow_one_small_cave_twice
                                && node.is_small_cave()
                                && !self.path_has_small_cave_twice(&path))
                    })
                    .map(|x| x.clone()) // don't like this
                    .collect::<Vec<Node>>();

                if reachable_nodes.len() == 0 {
                    // Dead path
                    continue;
                }

                for node in reachable_nodes.iter() {
                    let mut new_path = path.clone();
                    new_path.push(node.clone());
                    if node.is_end() {
                        completed_paths.push(new_path);
                    } else {
                        new_paths.push(new_path);
                    }
                }
            }
            paths = new_paths;
        }

        completed_paths.len() as i64
    }

    pub fn get_start(&self) -> Option<Node> {
        for (node, _) in self.0.iter() {
            if node.is_start() {
                return Some(node.clone());
            }
        }
        None
    }
    pub fn path_has_small_cave_twice(&self, path: &Vec<Node>) -> bool {
        for i in 1..path.len() {
            if path[i..].contains(&path[i - 1]) && path[i - 1].is_small_cave() {
                return true;
            }
        }
        false
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Graph, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    Graph::new(input_lines)
}

pub fn part_1(grid: &Graph) -> i64 {
    grid.find_num_paths_start_end_part2(false)
}

pub fn part_2(grid: &Graph) -> i64 {
    grid.find_num_paths_start_end_part2(true)
}

pub fn day12(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&parsed_data) as u64, part_2(&parsed_data) as u64)
}
