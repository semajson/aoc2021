use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone)]
pub struct Line {
    data: Vec<String>,
    first_illegal_char: Option<String>,
    chars_to_make_complete: Option<Vec<String>>,
}
impl Line {
    pub fn new(input_line: &str) -> Line {
        let input_line = input_line.to_owned();

        let line = input_line
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Line {
            data: line,
            first_illegal_char: None,
            chars_to_make_complete: None,
        }
    }

    pub fn workout_is_corrupted_or_incomplete(&mut self) {
        let bracket_matches: HashMap<&str, &str> =
            HashMap::from([("(", ")"), ("[", "]"), ("{", "}"), ("<", ">")]);

        let mut bracket_stack = vec![];

        for bracket in self.data.iter() {
            if bracket_matches.contains_key(bracket as &str) {
                bracket_stack.push(bracket);
            }
            // i don't like this, but can't think of a better way...
            else if (!bracket_stack.is_empty())
                && (bracket
                    == bracket_matches
                        .get(bracket_stack.last().unwrap() as &str)
                        .unwrap())
            {
                bracket_stack.pop().unwrap();
            } else {
                // Is corrupted
                self.first_illegal_char = Some(bracket.clone());
                return;
            }
        }

        // Must be incomplete - work out chars needed to make complete
        let mut chars_to_make_complete = vec![];
        for open_bracket in bracket_stack.iter().rev() {
            chars_to_make_complete.push(
                bracket_matches
                    .get(open_bracket as &str)
                    .unwrap()
                    .to_string(),
            );
        }
        self.chars_to_make_complete = Some(chars_to_make_complete.clone());
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

pub fn part_1(parsed_data: &[Line]) -> i64 {
    let mut parsed_data = parsed_data.to_owned();
    parsed_data
        .iter_mut()
        .for_each(|line| line.workout_is_corrupted_or_incomplete());

    // Work out the corrupted score
    let corrupted_bracket_scores: HashMap<&str, usize> =
        HashMap::from([(")", 3), ("]", 57), ("}", 1197), (">", 25137)]);
    let mut sum = 0;
    for line in parsed_data.into_iter() {
        if let Some(illegal_char) = line.first_illegal_char {
            sum += corrupted_bracket_scores.get(&illegal_char as &str).unwrap();
        }
    }
    sum as i64
}

pub fn part_2(parsed_data: &Vec<Line>) -> i64 {
    let mut parsed_data = parsed_data.to_owned();
    parsed_data
        .iter_mut()
        .for_each(|line| line.workout_is_corrupted_or_incomplete());

    // Work out the incomplete score
    let incomplete_bracket_scores: HashMap<&str, usize> =
        HashMap::from([(")", 1), ("]", 2), ("}", 3), (">", 4)]);
    let mut scores = vec![];
    for line in parsed_data.into_iter() {
        if let Some(chars_to_make_complete) = line.chars_to_make_complete {
            let mut score = 0;
            for char in chars_to_make_complete.iter() {
                score *= 5;
                score += incomplete_bracket_scores.get(char as &str).unwrap();
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();

    // Return the middle score
    let middle_index = ((scores.len() + 1) / 2) - 1;
    scores[middle_index] as i64
}

pub fn day10(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&parsed_data) as u64, part_2(&parsed_data) as u64)
}
