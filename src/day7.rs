use std::num;

pub trait CrabT {
    fn fuel_to_get_to_pos(&self, pos: i64) -> i64;
    fn get_pos(&self) -> i64;
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
    fn get_pos(&self) -> i64 {
        self.0
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
    fn get_pos(&self) -> i64 {
        self.0
    }
}

// fn parse_input_lines(
//     input_lines: &[String],
//     part1: bool,
// ) -> Result<Vec<impl CrabT>, num::ParseIntError> {
//     let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

//     if part1 {
//         let mut parsed_data = Vec::new();
//         for line in input_lines {
//             parsed_data.push(Crab1::new(line)?);
//         }
//         return Ok(parsed_data);
//     } else {
//         let mut parsed_data = Vec::new();
//         for line in input_lines {
//             parsed_data.push(Crab2::new(line)?);
//         }
//         return Ok(parsed_data);
//     }
// }
fn parse_input_lines_part1(input_lines: &[String]) -> Result<Vec<impl CrabT>, num::ParseIntError> {
    let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

    let mut parsed_data = Vec::new();
    for line in input_lines {
        parsed_data.push(Crab1::new(line)?);
    }
    return Ok(parsed_data);
}
fn parse_input_lines_part2(input_lines: &[String]) -> Result<Vec<impl CrabT>, num::ParseIntError> {
    let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

    let mut parsed_data = Vec::new();
    for line in input_lines {
        parsed_data.push(Crab2::new(line)?);
    }
    return Ok(parsed_data);
}

pub fn tot_fuel_at_pos(crabs: &Vec<impl CrabT>, pos: i64) -> i64 {
    let mut tot_fuel = 0;
    for crab in crabs {
        tot_fuel += crab.fuel_to_get_to_pos(pos);
    }

    tot_fuel
}

pub fn calc_optimal_pos(parsed_data: &Vec<impl CrabT>) -> i64 {
    // let crabs = (*parsed_data).clone();
    let crabs = parsed_data;

    // Find the new one
    let largest_post = crabs.iter().map(|crab| crab.get_pos()).max().unwrap();

    let mut prev_value = tot_fuel_at_pos(&crabs, 0);
    for i in 1..=largest_post {
        let cur_value = tot_fuel_at_pos(&crabs, i);
        if cur_value > prev_value {
            return prev_value;
        }
        prev_value = cur_value;
    }
    return prev_value;
}

pub fn day7(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines_part1(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });
    let part1 = calc_optimal_pos(&parsed_data) as u64;

    let parsed_data = parse_input_lines_part2(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });
    let part2 = calc_optimal_pos(&parsed_data) as u64;
    (part1, part2)
}
