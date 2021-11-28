// use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct FrequencyChange {
    change: i32,
}
impl FrequencyChange {
    pub fn new(mut line: &str) -> Result<FrequencyChange, io::Error> {
        let change = line.parse::<i32>().unwrap();
        Ok(FrequencyChange { change })
    }
}

pub fn parse_test_input_data() -> Result<Vec<FrequencyChange>, io::Error> {
    parse_data_from_file("inputs/test.txt")
}

pub fn parse_real_input_data() -> Result<Vec<FrequencyChange>, io::Error> {
    parse_data_from_file("inputs/real.txt")
}

fn parse_data_from_file(filename: &str) -> Result<Vec<FrequencyChange>, io::Error> {
    let mut parsed_data = Vec::new();

    let raw_input = fs::read_to_string(filename).unwrap();
    for line in raw_input.lines() {
        parsed_data.push(FrequencyChange::new(line).unwrap());
    }
    Ok(parsed_data)
}

pub fn part_1(mut parsed_data: &Vec<FrequencyChange>) -> () {
    let mut current_frequency = 0;
    for frequency_change in parsed_data {
        current_frequency += frequency_change.change;
    }
    println!("Part 1 answer is: {:?}", current_frequency);
}

pub fn part_2(mut parsed_data: &Vec<FrequencyChange>) -> () {
    let mut seen_frequencies = HashMap::new();
    let mut current_frequency = 0;

    loop {
        let mut seen_frequency_twice = false;
        for frequency_change in parsed_data {
            current_frequency += frequency_change.change;
            if seen_frequencies.contains_key(&current_frequency) {
                seen_frequency_twice = true;
                break;
            }
            seen_frequencies.insert(current_frequency, 1);
        }
        if seen_frequency_twice {
            break;
        }
    }

    println!("Part 2 answer is: {:?}", current_frequency);
}
