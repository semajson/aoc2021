use std::cmp::Ordering;
use std::num;

enum SnailfishNumberOption {
    Raw(i32),
    Pair(Vec<Box<SnailfishNumber>>),
}

// #[derive(Debug, Clone)]
pub struct SnailfishNumber {
    number: SnailfishNumberOption,
}
impl SnailfishNumber {
    pub fn new(line: &str) -> Result<SnailfishNumber, num::ParseIntError> {
        // [[[[4,3],4],4],[7,[[8,4],9]]]
        Ok(SnailfishNumber {
            number: SnailfishNumberOption::Raw(1),
        })
    }
}
fn build_snailfish_num(&str input) -> SnailfishNumber {
    // ok check ends and starts with []
    // remove them
    // then, look for the middle ,
    // walk through the string, have a record stack of [] seen,
    // stack is empty and seen a , - then we know it is the middle, so split again
}

fn parse_input_lines(
    raw_input_lines: &[String],
) -> Result<Vec<SnailfishNumber>, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    let snailfish_numbers = input_lines
        .iter()
        .map(|x| SnailfishNumber::new(x).unwrap())
        .collect::<_>();
    Ok(snailfish_numbers)
}

pub fn part_1(target_area: &Vec<SnailfishNumber>) -> i32 {
    println!("In here");
    0
}

pub fn part_2(target_area: &Vec<SnailfishNumber>) -> i32 {
    0
}

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, part_2(&encoded_data) as u64)
}
