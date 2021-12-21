use std::collections::HashMap;
use std::num;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Debug, Clone)]
pub struct Line {
    data: Vec<String>,
    is_incomplete: bool,
    is_corrupted: bool,
    first_illegal_char: Option<String>,
}
impl Line {
    pub fn new(input_line: &String) -> Line {
        let input_line = input_line.to_owned();

        let line = input_line
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Line {
            data: line,
            is_incomplete: false,
            is_corrupted: false,
            first_illegal_char: None,
        }
    }

    pub fn workout_is_corrupted_or_incomplete(&mut self) -> () {
        let bracket_matches: HashMap<&str, &str> =
            HashMap::from([("(", ")"), ("[", "]"), ("{", "}"), ("<", ">")]);

        let mut bracket_stack = vec![];

        for bracket in self.data.iter() {
            if bracket_matches.contains_key(bracket as &str) {
                bracket_stack.push(bracket);
            }
            // i don't like this, clean later..
            else if (!bracket_stack.is_empty())
                && (bracket
                    == bracket_matches
                        .get(bracket_stack.last().unwrap() as &str)
                        .unwrap())
            {
                bracket_stack.pop().unwrap();
            } else {
                self.is_corrupted = true;
                self.first_illegal_char = Some(bracket.clone());
                return;
            }
        }
        self.is_incomplete = true;
    }
}
fn parse_input_lines(raw_input_lines: &[String]) -> Result<Vec<Line>, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let mut parsed_data = Vec::new();
    for input_line in input_lines {
        parsed_data.push(Line::new(input_line));
    }
    Ok(parsed_data)
}

pub fn part_1(parsed_data: &Vec<Line>) -> i64 {
    let mut parsed_data = parsed_data.clone();
    parsed_data
        .iter_mut()
        .for_each(|line| line.workout_is_corrupted_or_incomplete());

    let bracket_matches: HashMap<&str, usize> =
        HashMap::from([(")", 3), ("]", 57), ("}", 1197), (">", 25137)]);
    let mut sum = 0;
    for line in parsed_data.into_iter() {
        if let Some(illegal_char) = line.first_illegal_char {
            sum += bracket_matches.get(&illegal_char as &str).unwrap();
        }
    }
    sum as i64
}

pub fn part_2(parsed_data: &Vec<Line>) -> i64 {
    println!("d");
    0
}

pub fn day10(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&parsed_data) as u64, part_2(&parsed_data) as u64)
}
