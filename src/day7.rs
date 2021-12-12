use std::num;

pub trait CrabT {
    fn fuel_to_get_to_pos(&self, pos: i64) -> i64;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crab1(i64);
impl Crab1 {
    pub fn new(line: &str) -> Result<Crab1, num::ParseIntError> {
        let value = line.parse::<i64>()?;
        Ok(Crab1(value))
    }
}
impl CrabT for Crab1 {
    fn fuel_to_get_to_pos(&self, pos: i64) -> i64 {
        (pos - self.0).abs()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Crab2(i64);
impl Crab2 {
    pub fn new(line: &str) -> Result<Crab2, num::ParseIntError> {
        let value = line.parse::<i64>()?;
        Ok(Crab2(value))
    }
}
impl CrabT for Crab2 {
    fn fuel_to_get_to_pos(&self, pos: i64) -> i64 {
        let horz_diff = (pos - self.0).abs();

        // use arth sum 1....horz_diff
        horz_diff * (1 + horz_diff) / 2
    }
}

fn parse_input_lines(
    input_lines: &[String],
    part1: bool,
) -> Result<Vec<Crab1>, num::ParseIntError> {
    let mut parsed_data = Vec::new();

    let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

    for line in input_lines {
        parsed_data.push(Crab1::new(line)?);
    }
    Ok(parsed_data)
}

pub fn tot_fuel_at_pos_part_2(crabs: &Vec<Crab1>, pos: i64) -> i64 {
    let mut tot_fuel = 0;
    for crab in crabs {
        let horz_diff = (pos - crab.0).abs();

        // use arth sum 1....horz_diff
        tot_fuel += horz_diff * (1 + horz_diff) / 2;
    }

    tot_fuel
}

pub fn tot_fuel_at_pos(crabs: &Vec<Crab1>, pos: i64) -> i64 {
    let mut tot_fuel = 0;
    for crab in crabs {
        tot_fuel += crab.fuel_to_get_to_pos(pos);
    }

    tot_fuel
}

pub fn part_1(parsed_data: &Vec<Crab1>) -> i64 {
    let crabs = (*parsed_data).clone();

    // Find the new one
    let largest_post = crabs.iter().map(|crab| crab.0).max().unwrap();

    let mut prev_value = tot_fuel_at_pos(&crabs, 0);
    for i in 1..=largest_post {
        let cur_value = tot_fuel_at_pos(&crabs, i);
        if cur_value > prev_value {
            return prev_value;
        }
        prev_value = cur_value;
    }
    return prev_value;

    // Sum all the fuel
}

pub fn part_2(parsed_data: &Vec<Crab1>) -> i64 {
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
    let parsed_test_data = parse_input_lines(input_lines, true).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });
    let part1 = part_1(&parsed_test_data) as u64;

    let parsed_test_data = parse_input_lines(input_lines, false).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });
    let part2 = part_2(&parsed_test_data) as u64;
    (part1, part2)
}
