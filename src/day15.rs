use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::num;

pub struct Direction {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    x: i64,
    y: i64,
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}
impl Node {
    pub fn reachable_nodes(&self) -> impl Iterator<Item = Node> {
        let me = self.clone();
        [
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: 0 },
            Direction { x: -1, y: 0 },
        ]
        .iter()
        .map(move |direction| {
            let new_x = direction.x + me.x;
            let new_y = direction.y + me.y;
            Node { x: new_x, y: new_y }
        })
    }
}

fn parse_input_lines(
    raw_input_lines: &[String],
    multiple: usize,
) -> Result<(HashMap<Node, i64>, Node, Node), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    let input_lines = input_lines.clone();

    let input_lines = input_lines
        .iter()
        .map(|row| {
            row.chars()
                .map(|char| char.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>(); // todo, better handle parse errors

    let top_left_x_len = input_lines.len();
    let top_left_y_len = input_lines[0].len();

    // Get start and end nodes
    let start = Node { x: 0, y: 0 };

    let max_y = (top_left_x_len * multiple) - 1;
    let max_x = (top_left_y_len * multiple) - 1;
    let end = Node {
        x: (max_x.clone() as i64),
        y: (max_y.clone() as i64),
    };

    // Get nodes + their costs
    let mut node_costs = HashMap::new();

    for x in 0..top_left_x_len {
        for y in 0..top_left_y_len {
            for x_grid_num in 0..multiple {
                for y_grid_num in 0..multiple {
                    let x_val = (x + x_grid_num * top_left_x_len) as i64;
                    let y_val = (y + y_grid_num * top_left_y_len) as i64;

                    let grid_num_max = (x_grid_num + y_grid_num) as i64;
                    let mut cost = input_lines[y][x] + grid_num_max;

                    while cost > 9 {
                        cost -= 9;
                    }

                    node_costs.insert(Node { x: x_val, y: y_val }, cost);
                }
            }
        }
    }

    Ok((node_costs, start, end))
}

#[derive(Debug, Clone)]
pub struct Info {
    cost_to_node: i64,
    prev_node: Option<Node>,
}

pub fn dijkstra_solve(node_costs: &HashMap<Node, i64>, start: &Node, end: &Node) -> i64 {
    let mut priority_queue = vec![];
    let mut finished_nodes = HashSet::new();

    // Initial value
    priority_queue.push((
        start.clone(),
        Info {
            cost_to_node: 0, // the starting position is never entered, so its risk is not counted
            prev_node: None,
        },
    ));

    while !priority_queue.is_empty() {
        // Get lowest value in the queue
        let mut min_index = 0;
        let mut min_cost_to_node = priority_queue[0].1.cost_to_node;
        for i in 1..priority_queue.len() {
            if priority_queue[i].1.cost_to_node < min_cost_to_node {
                min_cost_to_node = priority_queue[i].1.cost_to_node;
                min_index = i;
            }
        }
        let (node_to_eval, node_to_eval_info) = priority_queue.remove(min_index);

        if node_to_eval == *end {
            // Found the answer
            return node_to_eval_info.cost_to_node;
        }

        for reachable_node in node_to_eval.reachable_nodes() {
            if finished_nodes.contains(&reachable_node) {
                continue;
            }

            if let Some(reachable_node_cost) = node_costs.get(&reachable_node) {
                // Node exists
                let new_cost_to_node = node_to_eval_info.cost_to_node + reachable_node_cost;

                if let Some((_, existing_node_info)) = priority_queue
                    .iter_mut()
                    .find(|(node, _)| *node == reachable_node)
                {
                    // Node already in priority queue
                    if new_cost_to_node < existing_node_info.cost_to_node {
                        // Found shorter path
                        existing_node_info.cost_to_node = new_cost_to_node;
                        existing_node_info.prev_node = Some(node_to_eval.clone());
                    }
                } else {
                    // Insert new node into priority queue
                    priority_queue.push((
                        reachable_node,
                        Info {
                            cost_to_node: new_cost_to_node,
                            prev_node: Some(node_to_eval.clone()),
                        },
                    ));
                }
            }
        }

        finished_nodes.insert(node_to_eval.clone());
    }
    panic!("No answer found!");
}

pub fn part_1(node_costs: &HashMap<Node, i64>, start: &Node, end: &Node) -> i64 {
    dijkstra_solve(node_costs, start, end)
}

pub fn part_2(node_costs: &HashMap<Node, i64>, start: &Node, end: &Node) -> i64 {
    dijkstra_solve(node_costs, start, end)
}

pub fn day15(input_lines: &[String]) -> (u64, u64) {
    let (part1_node_costs, part1_start, part1_end) = parse_input_lines(input_lines, 1)
        .unwrap_or_else(|err| {
            panic!("Got error : {} , when trying to parse the input lines", err);
        });

    let (part2_node_costs, part2_start, part2_end) = parse_input_lines(input_lines, 5)
        .unwrap_or_else(|err| {
            panic!("Got error : {} , when trying to parse the input lines", err);
        });

    (
        part_1(&part1_node_costs, &part1_start, &part1_end) as u64,
        part_2(&part2_node_costs, &part2_start, &part2_end) as u64,
    )
}
