// use std::borrow::Borrow;
use std::fmt;
use std::num;

#[derive(Clone)]
pub struct Beacon {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone)]
pub struct Scanner {
    x: isize,
    y: isize,
    z: isize,
    beacons: Vec<Beacon>,
    num: usize,
}
impl Scanner {
    pub fn new(input_lines: Vec<&String>) -> Result<Scanner, num::ParseIntError> {
        let scanner_info = input_lines[0];
        let scanner_info = scanner_info.strip_prefix("--- scanner ").unwrap();
        let scanner_info = scanner_info.strip_suffix(" ---").unwrap();
        let scanner_num = scanner_info.parse::<usize>()?;

        let mut beacons = Vec::new();
        for line in input_lines[1..].iter() {
            let coords = line
                .split(',')
                .map(|x| x.parse::<isize>())
                .collect::<Result<Vec<isize>, num::ParseIntError>>();

            let coords = coords.unwrap();

            let beacon = Beacon {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            };
            beacons.push(beacon);
        }

        Ok(Scanner {
            x: 0,
            y: 0,
            z: 0,
            beacons,
            num: scanner_num,
        })
    }
}

#[derive(Clone)]
pub struct Map {
    scanners: Vec<Scanner>,
    all_beacons: Vec<Beacon>,
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Vec<Scanner>, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let mut scanners = Vec::new();
    let mut curr_scanner = Vec::new();
    for input_line in input_lines.into_iter() {
        if input_line.is_empty() {
            scanners.push(Scanner::new(curr_scanner).unwrap());
            curr_scanner = vec![];
        } else {
            curr_scanner.push(input_line);
        }
    }
    scanners.push(Scanner::new(curr_scanner).unwrap());
    curr_scanner = vec![];

    Ok(scanners)
}

pub fn part_1(numbers: &[Scanner]) -> i32 {
    let a = 0;
    println!("test");
    0
}

pub fn part_2(numbers: &[Scanner]) -> i32 {
    0
}

pub fn day19(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, part_2(&encoded_data) as u64)
}

#[test]
fn test_maybe_split_true() {
    0;
}
