use std::num;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crab(i64);
impl Crab {
    pub fn new(line: &str) -> Result<Crab, num::ParseIntError> {
        let value = line.parse::<i64>()?;
        Ok(Crab(value))
    }
}

fn parse_input_lines(input_lines: &[String]) -> Result<Vec<Crab>, num::ParseIntError> {
    let mut parsed_data = Vec::new();

    let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

    for line in input_lines {
        parsed_data.push(Crab::new(line)?);
    }
    Ok(parsed_data)
}

pub fn tot_fuel_at_pos_part_2(crabs: &Vec<Crab>, pos: i64) -> i64 {
    let mut tot_fuel = 0;
    for crab in crabs {
        let horz_diff = (pos - crab.0).abs();

        // use arth sum 1....horz_diff
        tot_fuel += horz_diff * (1 + horz_diff) / 2;
    }

    tot_fuel
}

pub fn tot_fuel_at_pos_part_1(crabs: &Vec<Crab>, pos: i64) -> i64 {
    let mut tot_fuel = 0;
    for crab in crabs {
        tot_fuel += (pos - crab.0).abs();
    }

    tot_fuel
}

pub fn part_1(parsed_data: &Vec<Crab>) -> i64 {
    let crabs = (*parsed_data).clone();

    // Find the new one
    let largest_post = crabs.iter().map(|crab| crab.0).max().unwrap();

    let mut prev_value = tot_fuel_at_pos_part_1(&crabs, 0);
    for i in 1..=largest_post {
        let cur_value = tot_fuel_at_pos_part_1(&crabs, i);
        if cur_value > prev_value {
            return prev_value;
        }
        prev_value = cur_value;
    }
    return prev_value;

    // Sum all the fuel
}

pub fn part_2(parsed_data: &Vec<Crab>) -> i64 {
    let crabs = (*parsed_data).clone();

    // Find the new one
    let largest_post = crabs.iter().map(|crab| crab.0).max().unwrap();

    let mut prev_value = tot_fuel_at_pos_part_2(&crabs, 0);
    for i in 1..=largest_post {
        let cur_value = tot_fuel_at_pos_part_2(&crabs, i);
        if cur_value > prev_value {
            return prev_value;
        }
        prev_value = cur_value;
    }
    return prev_value;

    // Sum all the fuel
}

pub fn day7(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1(&parsed_test_data) as u64,
        part_2(&parsed_test_data) as u64,
    )
}
