use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BinaryNum(Vec<u32>);

impl BinaryNum {
    pub fn new(line: &str) -> Result<BinaryNum, &'static str> {
        const RADIX: u32 = 10;

        let mut binary_num = Vec::new();
        for char in line.chars() {
            let char = char.to_digit(RADIX);

            if let Some(digit) = char {
                binary_num.push(digit);
            } else {
                return Err("Couldn't convert num ");
            }
        }
        Ok(BinaryNum(binary_num))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.0.iter()
    }

    pub fn to_dec(&self) -> usize {
        let bin_idx = self
            .0
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join("");

        usize::from_str_radix(&bin_idx, 2).unwrap()
    }
}

fn parse_input_lines(input_lines: &[String]) -> Result<Vec<BinaryNum>, &'static str> {
    let mut parsed_data = Vec::new();

    for line in input_lines {
        parsed_data.push(BinaryNum::new(line)?);
    }
    Ok(parsed_data)
}

fn count_digits(binary_nums: &[BinaryNum]) -> Vec<HashMap<u32, i32>> {
    let mut counts = Vec::new();
    for _ in 0..binary_nums[0].len() {
        counts.push(HashMap::new());
    }

    for binary_num in binary_nums {
        for (index, digit) in binary_num.iter().enumerate() {
            let count = counts[index].entry(*digit).or_insert(0);
            *count += 1;
        }
    }
    counts
}

fn calc_gamma(binary_nums: &[BinaryNum]) -> BinaryNum {
    // build a vec of all the counts at each digit
    let counts = count_digits(binary_nums);

    let mut gamma = Vec::new();
    for count in counts {
        if count.get(&0) > count.get(&1) {
            gamma.push(0_u32);
        } else {
            gamma.push(1_u32);
        }
    }
    BinaryNum(gamma)
}

fn calc_epsilon(gamma: &BinaryNum) -> BinaryNum {
    let mut epsilon = Vec::new();
    for digit in gamma.iter() {
        match digit {
            1 => epsilon.push(0),
            0 => epsilon.push(1),
            _ => panic!("Not expected!"),
        }
    }
    BinaryNum(epsilon)
}

pub fn part_1(parsed_data: &[BinaryNum]) -> usize {
    let gamma = calc_gamma(parsed_data);
    let epsilon = calc_epsilon(&gamma);

    println!("hello");

    gamma.to_dec() * epsilon.to_dec()
}

fn calc_oxygen_rating(binary_nums: &Vec<BinaryNum>) -> BinaryNum {
    let mut binary_nums = (*binary_nums).clone();

    let mut curr_index = 0;
    while binary_nums.len() > 1 {
        let counts = count_digits(&binary_nums);
        let curr_index_count = &counts[curr_index];
        let diff = curr_index_count[&0] - curr_index_count[&1];

        match diff.cmp(&0) {
            Ordering::Greater => {
                // 0 most common
                binary_nums = binary_nums
                    .into_iter()
                    .filter(|binary_num| binary_num.0[curr_index] == 0)
                    .collect::<Vec<_>>();
            }

            Ordering::Less => {
                // 1 most common
                binary_nums = binary_nums
                    .into_iter()
                    .filter(|binary_num| binary_num.0[curr_index] == 1)
                    .collect::<Vec<_>>();
            }
            Ordering::Equal => {
                // equal
                binary_nums = binary_nums
                    .into_iter()
                    .filter(|binary_num| binary_num.0[curr_index] == 1)
                    .collect::<Vec<_>>();
            }
        }
        curr_index += 1;
    }
    binary_nums[0].clone()
}

fn calc_co2_rating(binary_nums: &Vec<BinaryNum>) -> BinaryNum {
    let mut binary_nums = (*binary_nums).clone();

    let mut curr_index = 0;
    while binary_nums.len() > 1 {
        let counts = count_digits(&binary_nums);
        let curr_index_count = &counts[curr_index];
        let diff = curr_index_count[&0] - curr_index_count[&1];

        if diff > 0 {
            // 0 most common
            binary_nums = binary_nums
                .into_iter()
                .filter(|binary_num| binary_num.0[curr_index] == 1)
                .collect::<Vec<_>>();
        } else if diff < 0 {
            // 1 most common
            binary_nums = binary_nums
                .into_iter()
                .filter(|binary_num| binary_num.0[curr_index] == 0)
                .collect::<Vec<_>>();
        } else {
            // equal
            binary_nums = binary_nums
                .into_iter()
                .filter(|binary_num| binary_num.0[curr_index] == 0)
                .collect::<Vec<_>>();
        }
        curr_index += 1;
    }
    binary_nums[0].clone()
}

pub fn part_2(parsed_data: &Vec<BinaryNum>) -> usize {
    let oxygen_rating = calc_oxygen_rating(parsed_data);
    let calc_co2_rating = calc_co2_rating(parsed_data);

    oxygen_rating.to_dec() * calc_co2_rating.to_dec()
}

pub fn day3(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1(&parsed_test_data) as u64,
        part_2(&parsed_test_data) as u64,
    )
}
