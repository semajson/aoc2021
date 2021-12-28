use std::collections::HashMap;
use std::num;

pub struct PolymerMap(HashMap<String, String>);

fn parse_input_lines(
    raw_input_lines: &[String],
) -> Result<(String, PolymerMap), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    let mut input_lines = input_lines.clone();

    // Get initial state
    let initial_state = input_lines.remove(0);

    // Shift past empty line
    input_lines.remove(0);

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

fn for_polymer_pair_get_polymer_after_steps(
    polymer_pair: &str,
    polymer_map: &PolymerMap,
    steps: usize,
    polymer_step_cache: &mut HashMap<(String, usize), String>,
) -> String {
    if let Some(lookup_value) = polymer_step_cache.get(&(polymer_pair.to_string(), steps)) {
        return lookup_value.clone();
    }

    let new_polymer = polymer_pair[0..1].to_string()
        + polymer_map.0.get(polymer_pair).unwrap()
        + &polymer_pair[1..2].to_string();

    if steps == 1 {
        return new_polymer;
    }

    let output = get_polymer_after_steps(&new_polymer, polymer_map, steps - 1, polymer_step_cache);

    polymer_step_cache.insert((polymer_pair.to_string(), steps), output.clone());
    output
}

pub fn get_polymer_after_steps(
    initial_state: &str,
    polymer_map: &PolymerMap,
    steps: usize,
    polymer_step_cache: &mut HashMap<(String, usize), String>,
) -> String {
    // Get the resultant polymer
    //
    // Handle first pair differently to avoid double counting.
    let mut output = for_polymer_pair_get_polymer_after_steps(
        &initial_state[0..2],
        polymer_map,
        steps,
        polymer_step_cache,
    );

    // Do rest of pairs
    for i in 1..(initial_state.len() - 1) {
        // Remove the start of each polymer to avoid double counting
        output += &for_polymer_pair_get_polymer_after_steps(
            &initial_state[i..i + 2],
            polymer_map,
            steps,
            polymer_step_cache,
        )[1..]
            .to_string();
    }
    output
}

pub fn highest_count_minus_lowest_count(frequencies: &HashMap<String, u64>) -> u64 {
    let highest_freq = frequencies.iter().max_by_key(|entry| entry.1).unwrap().1;
    let lowest_freq = frequencies.iter().min_by_key(|entry| entry.1).unwrap().1;

    highest_freq - lowest_freq
}

pub fn calc_final_polymer_freq_diff_quick(
    initial_state: &str,
    polymer_map: &PolymerMap,
    steps: usize,
) -> u64 {
    // Get initial pair count
    let mut polymer_pair_count: HashMap<String, u64> = HashMap::new();
    for i in 0..(initial_state.len() - 1) {
        *polymer_pair_count
            .entry(initial_state[i..i + 2].to_string())
            .or_insert(0) += 1;
    }

    // Do steps
    for _ in 1..=steps {
        let mut polymer_pair_count_new: HashMap<String, u64> = HashMap::new();

        for (pair, count) in polymer_pair_count.iter_mut() {
            let first_pair = pair[0..1].to_string() + polymer_map.0.get(pair).unwrap();
            *polymer_pair_count_new.entry(first_pair).or_insert(0) += *count;

            let second_pair =
                polymer_map.0.get(pair).unwrap().to_string() + &pair[1..2].to_string();
            *polymer_pair_count_new.entry(second_pair).or_insert(0) += *count;
        }
        polymer_pair_count = polymer_pair_count_new;
    }

    // Get the frequencies of the letters
    let mut frequencies: HashMap<String, u64> = HashMap::new();
    for (pair, count) in polymer_pair_count.iter() {
        for char in pair.chars() {
            *frequencies.entry(char.to_string()).or_insert(0) += count;
        }
    }

    // Divide by 2 to avoid double counting
    for count in frequencies.values_mut() {
        *count = ((*count as f64) / (2_f64)).ceil() as u64;
    }

    highest_count_minus_lowest_count(&frequencies)
}

pub fn part_1(initial_state: &str, polymer_map: &PolymerMap) -> i64 {
    let mut polymer_step_cache: HashMap<(String, usize), String> = HashMap::new();
    let output = get_polymer_after_steps(initial_state, polymer_map, 10, &mut polymer_step_cache);

    let mut frequencies: HashMap<String, u64> = HashMap::new();
    for char in output.chars() {
        *frequencies.entry(char.to_string()).or_insert(0) += 1;
    }
    highest_count_minus_lowest_count(&frequencies) as i64
}

pub fn part_2(initial_state: &str, polymer_map: &PolymerMap) -> i64 {
    calc_final_polymer_freq_diff_quick(initial_state, polymer_map, 40) as i64
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
