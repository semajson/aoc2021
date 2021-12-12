use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputLine {
    digit_map: Vec<Digit>,
    output: Vec<Digit>,
}
impl InputLine {
    pub fn new(line: &str) -> Result<InputLine, &'static str> {
        let input = line.split("|").collect::<Vec<&str>>();

        if input.len() != 2 {
            return Err("Error: input line doesn't contain 2 things after parsing");
        }
        let digit_map = input[0]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| Digit(x.to_string()))
            .collect::<Vec<Digit>>();

        let output = input[1]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| Digit(x.to_string()))
            .collect::<Vec<Digit>>();

        Ok(InputLine { digit_map, output })
    }
}
fn parse_input_lines(input_lines: &[String]) -> Result<Vec<InputLine>, &'static str> {
    let mut parsed_data = Vec::new();

    // let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

    for line in input_lines.iter() {
        parsed_data.push(InputLine::new(line)?);
    }
    Ok(parsed_data)
}

pub fn part_1(parsed_data: &Vec<InputLine>, real_digit_map: &HashMap<i32, Digit>) -> i64 {
    // let mut InputLinees = (*parsed_data).clone();
    let real_digit_lens = real_digit_map
        .iter()
        .map(|z| z.1 .0.len())
        .collect::<Vec<usize>>();

    let real_digit_lens_only_once = real_digit_lens
        .iter()
        .filter(|x| real_digit_lens.iter().filter(|y| y == x).count() == 1)
        .map(|x| *x)
        .collect::<Vec<usize>>();

    let mut count_knowns = 0;
    for line in parsed_data.iter() {
        for digit in line.output.iter() {
            if real_digit_lens_only_once.contains(&digit.0.len()) {
                count_knowns += 1;
            }
        }
    }

    count_knowns
}
pub fn part_2(parsed_data: &Vec<InputLine>, real_digit_map: &HashMap<i32, Digit>) -> i64 {
    let mut count = 0;
    for line in parsed_data.iter() {
        count += find_output(line, real_digit_map);
    }

    count
}

fn find_output(InputLine: &InputLine, real_digit_map: &HashMap<i32, Digit>) -> i64 {
    // Find the unique lengths
    let real_digit_lens = real_digit_map
        .iter()
        .map(|z| z.1 .0.len())
        .collect::<Vec<usize>>();

    let real_digit_lens_only_once = real_digit_lens
        .iter()
        .filter(|x| real_digit_lens.iter().filter(|y| y == x).count() == 1)
        .map(|x| *x)
        .collect::<Vec<usize>>();

    // setup possible encoded values for each char
    let possible_vals_for_digit = vec!["a", "b", "c", "d", "e", "f", "g"];
    let mut current_possible_vals = HashMap::from([
        ("a", possible_vals_for_digit.clone()),
        ("b", possible_vals_for_digit.clone()),
        ("c", possible_vals_for_digit.clone()),
        ("d", possible_vals_for_digit.clone()),
        ("e", possible_vals_for_digit.clone()),
        ("f", possible_vals_for_digit.clone()),
        ("g", possible_vals_for_digit.clone()),
    ]);

    // Do initial trim of possible values based on the unique lengths
    for encoded_digit in InputLine.digit_map.iter() {
        if real_digit_lens_only_once.contains(&encoded_digit.0.len()) {
            let mut actual_value = None;
            for (_, v) in real_digit_map {
                if v.0.len() == encoded_digit.0.len() {
                    actual_value = Some(v);
                }
            }

            reduce_possible_values(
                &encoded_digit,
                actual_value.unwrap(),
                &mut current_possible_vals,
            )
        }
    }

    // Now sudoku the rest
    while !solved_vals(&current_possible_vals) {
        let mut new_current_possible_vals = current_possible_vals.clone();

        for (_, option) in current_possible_vals.iter_mut() {
            if option.len() == 1 {
                // ensure no other possible values have this

                for k in vec!["a", "b", "c", "d", "e", "f", "g"] {
                    if k != option[0] {
                        let mut current_pos_values_for_char =
                            new_current_possible_vals.get_mut(&k as &str).unwrap();

                        current_pos_values_for_char.retain(|x| *x != option[0]);
                    }
                }
                break;
            }
        }
        current_possible_vals = new_current_possible_vals;
    }

    0
}

fn reduce_possible_values(
    encoded_value: &Digit,
    actual_value: &Digit,
    current_possible_vals: &mut HashMap<&str, Vec<&str>>,
) -> () {
    assert!(encoded_value.0.len() == actual_value.0.len());
    //
    for k in vec!["a", "b", "c", "d", "e", "f", "g"] {
        if actual_value.0.contains(k) {
            // Ensure that the char has valid guesses
            let mut current_pos_values_for_char =
                current_possible_vals.get_mut(&k as &str).unwrap();

            current_pos_values_for_char.retain(|x| encoded_value.0.contains(x));
        } else {
            // Ensure that no other ones chars have this char
            let mut current_pos_values_for_char =
                current_possible_vals.get_mut(&k as &str).unwrap();

            current_pos_values_for_char.retain(|x| !encoded_value.0.contains(x));
        }
    }
    println!("Done");
}

fn solved_vals(possible_vals: &HashMap<&str, Vec<&str>>) -> bool {
    for (k, v) in possible_vals {
        if v.len() > 1 {
            return false;
        }
    }
    true
}

pub fn day8(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    let real_digit_map: HashMap<i32, Digit> = HashMap::from([
        (0, Digit("abcefg".to_string())),
        (1, Digit("cf".to_string())),
        (2, Digit("acdeg".to_string())),
        (3, Digit("acdfg".to_string())),
        (4, Digit("bcdf".to_string())),
        (5, Digit("abdfg".to_string())),
        (6, Digit("abdefg".to_string())),
        (7, Digit("acf".to_string())),
        (8, Digit("abcdefg".to_string())),
        (9, Digit("abcdfg".to_string())),
    ]);

    (0, part_2(&parsed_test_data, &real_digit_map) as u64)
}
