use std::num;

#[derive(Debug)]
pub struct Measurement(i32);
impl Measurement {
    pub fn new(line: &str) -> Result<Measurement, num::ParseIntError> {
        let value = line.parse::<i32>()?;
        Ok(Measurement(value))
    }
}

fn parse_input_lines(input_lines: &[String]) -> Result<Vec<Measurement>, num::ParseIntError> {
    let mut parsed_data = Vec::new();

    for line in input_lines {
        parsed_data.push(Measurement::new(line)?);
    }
    Ok(parsed_data)
}

pub fn part_1(parsed_data: &Vec<Measurement>) -> i32 {
    let raw_data = parsed_data
        .iter()
        .map(|measurement| measurement.0)
        .collect::<Vec<_>>();
    find_num_increasing(&raw_data)
}

pub fn part_2(parsed_data: &Vec<Measurement>) -> i32 {
    // First, find the rolling sum
    let num_to_sum = 3;

    let mut rolling_sum = vec![0; parsed_data.len() - (num_to_sum - 1)];
    let rolling_sum_len = rolling_sum.len();

    for i in 0..rolling_sum_len {
        let mut sum = 0;
        for j in 0..num_to_sum {
            sum += parsed_data[i + j].0;
        }
        rolling_sum[i] = sum;
    }

    // Now find the num of increasing values
    find_num_increasing(&rolling_sum)
}

fn find_num_increasing(data: &Vec<i32>) -> i32 {
    let mut last_measurement = None;
    let mut num_increases = 0;

    for measurement in data {
        if let Some(last) = last_measurement {
            if measurement > last {
                num_increases += 1;
            }
        }
        last_measurement = Some(measurement);
    }
    num_increases
}

pub fn day1(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1(&parsed_test_data) as u64,
        part_2(&parsed_test_data) as u64,
    )
}
