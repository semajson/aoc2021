use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::num;

pub struct PolymerMap(HashMap<String, String>);

fn parse_input_lines(
    raw_input_lines: &[String],
) -> Result<(String, PolymerMap), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    let mut input_lines = input_lines.clone();

    // Get initial state
    let initial_state = input_lines.remove(0);

    // shift past plan line
    let mut line = input_lines.remove(0);

    // Get polymer map
    let mut polymer_map = HashMap::new();
    loop {
        let line = input_lines.remove(0);
        let line = line.split(" -> ").collect::<Vec<&str>>();

        polymer_map.insert(line[0].to_string(), line[1].to_string());

        if input_lines.is_empty() {
            break;
        }
    }

    Ok((initial_state.to_string(), PolymerMap(polymer_map)))
}

fn get_polymer_after_steps(
    polymer_pair: &str,
    polymer_map: &PolymerMap,
    steps: usize,
    polymer_cache: &mut HashMap<(String, usize), String>,
) -> String {
    if let Some(lookup_vale) = polymer_cache.get(&(polymer_pair.to_string(), steps)) {
        return lookup_vale.clone();
    }

    let new_polymer = polymer_pair[0..1].to_string()
        + polymer_map.0.get(polymer_pair).unwrap()
        + &polymer_pair[1..2].to_string();

    if steps == 1 {
        return new_polymer;
    }

    let mut output =
        get_polymer_after_steps(&new_polymer[0..2], polymer_map, steps - 1, polymer_cache);

    for i in 1..(new_polymer.len() - 1) {
        output += &get_polymer_after_steps(
            &new_polymer[i..i + 2],
            polymer_map,
            steps - 1,
            polymer_cache,
        )[1..]
            .to_string();
    }
    polymer_cache.insert((polymer_pair.to_string(), steps), output.clone());
    output
}

pub fn calc_required_polymer(
    initial_state: &String,
    polymer_map: &PolymerMap,
    steps: usize,
) -> String {
    let mut polymer_cache: HashMap<(String, usize), String> = HashMap::new();
    let mut output = "".to_string();
    for i in 0..(initial_state.len() - 1) {
        output += &get_polymer_after_steps(
            &initial_state[i..i + 2],
            polymer_map,
            steps,
            &mut polymer_cache,
        );
    }
    output
}

pub fn part_1(initial_state: &String, polymer_map: &PolymerMap) -> i64 {
    println!(
        "Output after 1: {}",
        calc_required_polymer(initial_state, polymer_map, 1)
    );
    println!(
        "Output after 2: {}",
        calc_required_polymer(initial_state, polymer_map, 2)
    );
    println!(
        "Output after 3: {}",
        calc_required_polymer(initial_state, polymer_map, 3)
    );

    let output = calc_required_polymer(initial_state, polymer_map, 10);

    let mut frequency: HashMap<String, u32> = HashMap::new();
    for char in output.chars() {
        // word is a &str
        // let key = char.to_string();
        *frequency.entry(char.to_string()).or_insert(0) += 1; // word does not live long enough
    }
    let highest_freq = frequency.iter().max_by_key(|entry| entry.1).unwrap().1;
    let lowest_freq = frequency.iter().min_by_key(|entry| entry.1).unwrap().1;
    (highest_freq - lowest_freq) as i64
}

pub fn part_2(initial_state: &String, polymer_map: &PolymerMap) -> i64 {
    let output = calc_required_polymer(initial_state, polymer_map, 40);

    let mut frequency: HashMap<String, u32> = HashMap::new();
    for char in output.chars() {
        // word is a &str
        // let key = char.to_string();
        *frequency.entry(char.to_string()).or_insert(0) += 1; // word does not live long enough
    }
    let highest_freq = frequency.iter().max_by_key(|entry| entry.1).unwrap().1;
    let lowest_freq = frequency.iter().min_by_key(|entry| entry.1).unwrap().1;
    (highest_freq - lowest_freq) as i64
}

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    let (initial_state, polymer_map) = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });

    (
        part_1(&initial_state, &polymer_map) as u64,
        part_2(&initial_state, &polymer_map) as u64,
    )
}
