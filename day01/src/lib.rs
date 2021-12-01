// use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Measurement {
    value: i32,
}
impl Measurement {
    pub fn new(mut line: &str) -> Result<Measurement, io::Error> {
        let value = line.parse::<i32>().unwrap();
        Ok(Measurement { value })
    }
}

pub fn parse_test_input_data() -> Result<Vec<Measurement>, io::Error> {
    parse_data_from_file("inputs/test.txt")
}

pub fn parse_real_input_data() -> Result<Vec<Measurement>, io::Error> {
    parse_data_from_file("inputs/real.txt")
}

fn parse_data_from_file(filename: &str) -> Result<Vec<Measurement>, io::Error> {
    let mut parsed_data = Vec::new();

    let raw_input = fs::read_to_string(filename).unwrap();
    for line in raw_input.lines() {
        parsed_data.push(Measurement::new(line).unwrap());
    }
    Ok(parsed_data)
}

fn find_num_increasing(data: &Vec<i32>) -> i32 {
    let mut last_measurement = None;
    let mut num_increases = 0;
    let mut num_decreases = 0;
    let mut num_unchanged = 0;

    for measurement in data {
        if last_measurement == None {
        } else if measurement == last_measurement.unwrap() {
            num_unchanged += 1;
        } else if measurement >= last_measurement.unwrap() {
            num_increases += 1;
        } else if measurement <= last_measurement.unwrap() {
            num_decreases += 1;
        }
        last_measurement = Some(measurement);
    }
    num_increases
}

pub fn part_1(mut parsed_data: &Vec<Measurement>) -> () {
    let raw_data = parsed_data
        .iter()
        .map(|measurement| measurement.value)
        .collect::<Vec<_>>();
    println!("Part 1 answer is: {:?}", find_num_increasing(&raw_data));
}

pub fn part_2(mut parsed_data: &Vec<Measurement>) -> () {
    // First, find the rolling sum
    let num_to_sum = 3;

    let mut rolling_sum = vec![0; parsed_data.len() - (num_to_sum - 1)];
    let len = rolling_sum.len();

    for i in 0..len {
        let mut total = 0;
        for j in 0..num_to_sum {
            total += parsed_data[i + j].value;
        }
        rolling_sum[i] = total;
    }
    println!("Part 2 answer is: {:?}", find_num_increasing(&rolling_sum));
}
