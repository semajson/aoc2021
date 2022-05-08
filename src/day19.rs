// use std::borrow::Borrow;
use ndarray::{arr1, arr2, Array1};
use std::fmt;
use std::num;

#[derive(Clone, Debug)]
pub struct Beacon(Array1<isize>);

#[derive(Clone)]
pub struct Scanner {
    location: Array1<isize>,
    beacons_variations: Vec<Vec<Beacon>>,
    curr_beacons_variation: usize,
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
                .collect::<Result<Array1<isize>, num::ParseIntError>>();

            let beacon = Beacon(coords.unwrap());
            beacons.push(beacon);
        }

        let beacons_variations = Scanner::get_variations(beacons);

        Ok(Scanner {
            location: arr1(&[0, 0, 0]),
            beacons_variations,
            curr_beacons_variation: 0,
            num: scanner_num,
        })
    }
    pub fn get_variations(base_beacons: Vec<Beacon>) -> Vec<Vec<Beacon>> {
        let mut variations = Vec::new();

        variations.push(base_beacons.clone());

        println!("test{:?}", base_beacons);

        let mut curr_variation = base_beacons;

        let rot_x = arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]]);
        let relf_x = arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]]);

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.0.dot(&rot_x);
                new_variation.push(Beacon(new_beacon));
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        // Do X reflection
        let mut new_variation = Vec::new();
        for beacon in curr_variation.into_iter() {
            let new_beacon = beacon.0.dot(&relf_x);
            new_variation.push(Beacon(new_beacon));
        }
        curr_variation = new_variation.clone();
        variations.push(new_variation.clone());

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.0.dot(&rot_x);
                new_variation.push(Beacon(new_beacon));
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        variations
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
