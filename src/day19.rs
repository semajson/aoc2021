// use std::borrow::Borrow;
use ndarray::{arr1, arr2, Array1};
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;
use std::num;

// #[derive(Clone, Debug)]
// pub struct Beacon(Array1<isize>);

// works but slow
// ideas to increase speed:
// - get rid of vecs, use all ndarrays instead
// - identify any duplicated work
// - look at other peoples answer

#[derive(Clone)]
pub struct Scanner {
    beacons_variations: Vec<Vec<Array1<isize>>>,
    verified_beacons: HashSet<Array1<isize>>,
    num: usize,
    location: Array1<isize>,
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

            let beacon = coords.unwrap();
            beacons.push(beacon);
        }

        let beacons_variations = Scanner::get_variations(beacons);

        // assume for now
        let verified_beacons = HashSet::from_iter(beacons_variations[0].clone().into_iter());

        Ok(Scanner {
            beacons_variations,
            verified_beacons,
            num: scanner_num,
            location: Array1::from_vec(vec![0, 0, 0]),
        })
    }
    pub fn get_variations(base_beacons: Vec<Vec<isize>>) -> Vec<Vec<Array1<isize>>> {
        let mut variations = Vec::new();

        // convert to ndarry vecs for dot product stuff
        let base_beacons = base_beacons
            .into_iter()
            .map(|x| Array1::from_vec(x))
            .collect::<Vec<Array1<isize>>>();

        // println!("test{:#?}", base_beacons);

        let mut curr_variation = base_beacons.clone();
        variations.push(base_beacons.clone());

        let rot_x_90 = arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]]);
        let rot_z_180 = arr2(&[[-1, 0, 0], [0, -1, 0], [0, 0, 1]]);
        let rot_z_90 = arr2(&[[0, -1, 0], [1, 0, 0], [0, 0, 1]]);
        let rot_y_90 = arr2(&[[0, 0, 1], [0, 1, 0], [-1, 0, 0]]);

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.dot(&rot_x_90);
                new_variation.push(new_beacon);
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        let mut new_variation = Vec::new();
        for beacon in curr_variation.into_iter() {
            let new_beacon = beacon.dot(&rot_z_180);
            new_variation.push(new_beacon);
        }
        curr_variation = new_variation.clone();
        variations.push(new_variation.clone());

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.dot(&rot_x_90);
                new_variation.push(new_beacon);
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        curr_variation = base_beacons.clone();
        let mut new_variation = vec![];
        for beacon in curr_variation.into_iter() {
            let new_beacon = beacon.dot(&rot_z_90);
            new_variation.push(new_beacon);
        }
        curr_variation = new_variation.clone();
        variations.push(new_variation.clone());

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.dot(&rot_x_90);
                new_variation.push(new_beacon);
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        let mut new_variation = Vec::new();
        for beacon in curr_variation.into_iter() {
            let new_beacon = beacon.dot(&rot_z_180);
            new_variation.push(new_beacon);
        }
        curr_variation = new_variation.clone();
        variations.push(new_variation.clone());

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.dot(&rot_x_90);
                new_variation.push(new_beacon);
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        curr_variation = base_beacons.clone();
        let mut new_variation = vec![];
        for beacon in curr_variation.into_iter() {
            let new_beacon = beacon.dot(&rot_y_90);
            new_variation.push(new_beacon);
        }
        curr_variation = new_variation.clone();
        variations.push(new_variation.clone());

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.dot(&rot_x_90);
                new_variation.push(new_beacon);
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        let mut new_variation = Vec::new();
        for beacon in curr_variation.into_iter() {
            let new_beacon = beacon.dot(&rot_z_180);
            new_variation.push(new_beacon);
        }
        curr_variation = new_variation.clone();
        variations.push(new_variation.clone());

        // Do 3 x rotations
        for _ in 0..3 {
            let mut new_variation = Vec::new();
            for beacon in curr_variation.into_iter() {
                let new_beacon = beacon.dot(&rot_x_90);
                new_variation.push(new_beacon);
            }
            curr_variation = new_variation.clone();
            variations.push(new_variation.clone());
        }

        // convert back to pure vecs
        // let variations = variations
        //     .iter()
        //     .map(|x| x.iter().map(|y| y.to_vec()).collect::<Vec<Vec<isize>>>())
        //     .collect::<Vec<Vec<Vec<isize>>>>();

        // let variations = variations
        //     .into_iter()
        //     .map(|x| HashSet::from_iter(x.into_iter()))
        //     .collect::<Vec<HashSet<Vec<isize>>>>();
        variations
    }
}

#[derive(Clone)]
pub struct Map {
    unmatched_scanners: Vec<Scanner>,
    edge_matched_scanners: Vec<Scanner>,
    tried_matched_scanners: Vec<Scanner>,
    verified_beacons: HashSet<Array1<isize>>,
}
impl Map {
    pub fn new(mut unmatched_scanners: Vec<Scanner>) -> Map {
        // assume the first scanner is correct, add it to the verified beacons
        let matched_scanner = unmatched_scanners.pop().unwrap();
        let verified_beacons = matched_scanner.beacons_variations[0].clone();

        // let test = vec!["sdf", "sdf"];
        let verified_beacons = HashSet::from_iter(verified_beacons.into_iter());

        Map {
            unmatched_scanners: unmatched_scanners,
            edge_matched_scanners: vec![matched_scanner],
            tried_matched_scanners: vec![],
            verified_beacons: verified_beacons,
        }
    }

    pub fn fill_in_map(&mut self) -> () {
        println!("Starging fill_in_map");
        while self.unmatched_scanners.len() > 0 {
            assert!((self.edge_matched_scanners.len() > 0));
            let mut added_scanner = false;

            // Store of the current edge scanners
            let mut edge_matched_scanners = self.edge_matched_scanners.clone();
            self.edge_matched_scanners = vec![];

            for edge_scanner in edge_matched_scanners.iter_mut() {
                // store off current unmachted_scanners:
                let mut unmatched_scanners = self.unmatched_scanners.clone();

                let mut new_matched_scanners = vec![];
                for unmatched_scanner in unmatched_scanners.iter_mut() {
                    if self.can_see_scanner(edge_scanner, unmatched_scanner) {
                        added_scanner = true;
                        new_matched_scanners.push(unmatched_scanner.num);
                    }
                }

                // remove unmachted scanners
                for matched_scanner_num in new_matched_scanners.into_iter() {
                    let index = self
                        .unmatched_scanners
                        .iter()
                        .position(|x| x.num == matched_scanner_num)
                        .unwrap();

                    self.unmatched_scanners.remove(index);
                }
            }

            self.tried_matched_scanners
                .append(&mut edge_matched_scanners);

            assert!((added_scanner) || (self.unmatched_scanners.len() == 0));
            println!(
                "Remaining unmatched scanners: {}",
                self.unmatched_scanners.len()
            );
        }
    }

    // pub fn added_scanner_to_map(&mut self, unmatched_scanner: &mut Scanner) -> bool {
    //     // Loop through variations looking for a match
    //     for beacon_variation in unmatched_scanner.beacons_variations.iter() {
    //         for unmatched_beacon in beacon_variation.iter() {
    //             for verified_beacon in self.verified_beacons.iter() {
    //                 // assume these two beacons are the same, and check all others for matches
    //                 let offset = vec_a_minus_b(verified_beacon, unmatched_beacon);

    //                 let mut match_count = 0;
    //                 for other_unamchted_beacon in beacon_variation.iter() {
    //                     let other_unamchted_beacon = vec_a_add_b(other_unamchted_beacon, &offset);

    //                     if self.verified_beacons.contains(&other_unamchted_beacon) {
    //                         match_count += 1;
    //                     }
    //                 }
    //                 // we should always have at least 1 match
    //                 assert!(match_count > 0);

    //                 if match_count >= 12 {
    //                     // got a match!

    //                     // remove it from the unmatched list
    //                     let index = self
    //                         .unmatched_scanners
    //                         .iter()
    //                         .position(|x| {
    //                             *x.beacons_variations == unmatched_scanner.beacons_variations
    //                         })
    //                         .unwrap();

    //                     self.unmatched_scanners.remove(index);

    //                     // add the beacon coords to the verified beacon list
    //                     for other_unamchted_beacon in beacon_variation.iter() {
    //                         let found_beacon = vec_a_add_b(other_unamchted_beacon, &offset);
    //                         if !self.verified_beacons.contains((&found_beacon)) {
    //                             self.verified_beacons.insert(found_beacon);
    //                         }
    //                     }
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     // No match :(
    //     return false;
    // }

    pub fn can_see_scanner(
        &mut self,
        edge_matched_scanner: &mut Scanner,
        unmatched_scanner: &mut Scanner,
    ) -> bool {
        // Loop through variations looking for a match
        for beacon_variation in unmatched_scanner.beacons_variations.iter() {
            for unmatched_beacon in beacon_variation.iter() {
                // see if this unmatched scanners matches this matched scanner
                for verified_beacon in edge_matched_scanner.verified_beacons.iter() {
                    // assume these two beacons are the same, and check all others for matches
                    // let offset = vec_a_minus_b(verified_beacon, unmatched_beacon);
                    let offset = verified_beacon - unmatched_beacon;

                    let mut match_count = 0;
                    for other_unamchted_beacon in beacon_variation.iter() {
                        let other_unamchted_beacon = other_unamchted_beacon + &offset;

                        if self.verified_beacons.contains(&other_unamchted_beacon) {
                            match_count += 1;
                        }
                    }
                    // we should always have at least 1 match
                    assert!(match_count > 0);

                    if match_count >= 12 {
                        // got a match!

                        // println!("Unmatched scanner found number: {}", unmatched_scanner.num);

                        // add it to the matched_scanner list
                        let mut translated_beacons = vec![];
                        for other_unamchted_beacon in beacon_variation.iter() {
                            translated_beacons.push(other_unamchted_beacon + &offset);
                        }
                        unmatched_scanner.verified_beacons =
                            HashSet::from_iter(translated_beacons.clone().into_iter());
                        unmatched_scanner.location = offset.clone();
                        self.edge_matched_scanners.push(unmatched_scanner.clone());

                        // add the beacon coords to the verified beacon list
                        for translated_beacon in translated_beacons.into_iter() {
                            if !self.verified_beacons.contains(&translated_beacon) {
                                self.verified_beacons.insert(translated_beacon);
                            }
                        }

                        // removing the scanner from the unmatched list is done in the calling function
                        return true;
                    }
                }
            }
        }

        // No match :(
        return false;
    }

    pub fn max_distance(&self) -> usize {
        let mut max_distance = 0;
        for a in self.tried_matched_scanners.iter() {
            for b in self.tried_matched_scanners.iter() {
                let distance = manhat_distance(&a.location, &b.location);
                if distance > max_distance {
                    max_distance = distance;
                }
            }
        }
        max_distance as usize
    }
}

pub fn manhat_distance(a: &Array1<isize>, b: &Array1<isize>) -> usize {
    let x_diff = a[0] - b[0];
    let y_diff = a[1] - b[1];
    let z_diff = a[2] - b[2];

    return (x_diff.abs() + y_diff.abs() + z_diff.abs()) as usize;
}

fn vec_a_add_b(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    let a = Array1::from_vec(a.clone());
    let b = Array1::from_vec(b.clone());
    let result = a + b;
    return result.to_vec();
}

fn vec_a_minus_b(a: &Vec<isize>, b: &Vec<isize>) -> Vec<isize> {
    let a = Array1::from_vec(a.clone());
    let b = Array1::from_vec(b.clone());
    let result = a - b;
    return result.to_vec();
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
    // horrible hack
    if true {
        return 1;
    }
    let a = 0;

    let mut map = Map::new(numbers.to_vec());
    map.fill_in_map();

    map.verified_beacons.len() as i32
}

pub fn part_2(numbers: &[Scanner]) -> i32 {
    let a = 0;

    let mut map = Map::new(numbers.to_vec());
    map.fill_in_map();

    map.max_distance() as i32
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
