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

pub fn part_1(parsed_data: &Vec<InputLine>, real_digit_map: &HashMap<Digit, i32>) -> i64 {
    // let mut InputLinees = (*parsed_data).clone();
    let real_digit_lens = real_digit_map
        .iter()
        .map(|z| z.0 .0.len())
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

pub fn part_2(parsed_data: &Vec<InputLine>, real_digit_map: &HashMap<Digit, i32>) -> i64 {
    let mut count = 0;
    for line in parsed_data.iter() {
        count += find_output(line, real_digit_map);
    }

    count
}

fn find_output(InputLine: &InputLine, real_digit_map: &HashMap<Digit, i32>) -> i64 {
    // Find the unique lengths
    let real_digit_lens = real_digit_map
        .iter()
        .map(|z| z.0 .0.len())
        .collect::<Vec<usize>>();

    let real_digit_lens_only_once = real_digit_lens
        .iter()
        .filter(|x| real_digit_lens.iter().filter(|y| y == x).count() == 1)
        .map(|x| *x)
        .collect::<Vec<usize>>();

    // setup possible encoded values for each char
    let possible_actual_values = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
        "g".to_string(),
    ];
    let mut encoded_to_actual = HashMap::from([
        ("a".to_string(), possible_actual_values.clone()),
        ("b".to_string(), possible_actual_values.clone()),
        ("c".to_string(), possible_actual_values.clone()),
        ("d".to_string(), possible_actual_values.clone()),
        ("e".to_string(), possible_actual_values.clone()),
        ("f".to_string(), possible_actual_values.clone()),
        ("g".to_string(), possible_actual_values.clone()),
    ]);

    // Do initial trim of possible values based on the unique lengths
    for encoded_digit in InputLine.digit_map.iter() {
        if real_digit_lens_only_once.contains(&encoded_digit.0.len()) {
            let mut actual_value = None;
            for (k, _) in real_digit_map {
                if k.0.len() == encoded_digit.0.len() {
                    actual_value = Some(k);
                }
            }

            reduce_possible_values(
                &encoded_digit,
                actual_value.unwrap(),
                &mut encoded_to_actual,
            )
        }
    }

    // Now sudoku the rest
    while !solved_vals(&encoded_to_actual) {
        let mut new_encoded_to_actual = encoded_to_actual.clone();

        for (_, option) in encoded_to_actual.iter_mut() {
            if option.len() == 1 {
                // ensure no other possible values have this

                for k in vec!["a", "b", "c", "d", "e", "f", "g"] {
                    if k != option[0] {
                        let mut current_pos_values_for_char =
                            new_encoded_to_actual.get_mut(&k as &str).unwrap();

                        current_pos_values_for_char.retain(|x| *x != option[0]);
                    }
                }
                break;
            }
        }
        encoded_to_actual = new_encoded_to_actual;
    }

    return decode_digit(&encoded_to_actual, &InputLine.output[0], real_digit_map) * 1000
        + decode_digit(&encoded_to_actual, &InputLine.output[1], real_digit_map) * 100
        + decode_digit(&encoded_to_actual, &InputLine.output[2], real_digit_map) * 10
        + decode_digit(&encoded_to_actual, &InputLine.output[3], real_digit_map) * 1;
}

fn decode_digit(
    encoded_to_actual: &HashMap<String, Vec<String>>,
    digit: &Digit,
    real_digit_map: &HashMap<Digit, i32>,
) -> i64 {
    let mut new_digit = "".to_string();
    for char in digit.0.chars() {
        let new_char = encoded_to_actual.get(&char.to_string()).unwrap()[0].clone();
        new_digit.push(new_char.chars().nth(0).unwrap());
    }

    *(real_digit_map.get(&Digit(new_digit)).unwrap()) as i64
}

fn reduce_possible_values(
    encoded_value: &Digit,
    actual_value: &Digit,
    encoded_to_actual: &mut HashMap<String, Vec<String>>,
) -> () {
    assert!(encoded_value.0.len() == actual_value.0.len());
    for (index, encoded_char) in encoded_value.0.chars().enumerate() {
        let actual_char = &actual_value.0.chars().nth(index).unwrap();

        let encoded_key = encoded_char.to_string();
        let new_vec = vec![actual_char.to_string()];

        encoded_to_actual.insert(encoded_key, new_vec);

        println!("hello");
    }

    println!("Done");
}

fn solved_vals(possible_vals: &HashMap<String, Vec<String>>) -> bool {
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

    let real_digit_map: HashMap<Digit, i32> = HashMap::from([
        (Digit("abcefg".to_string()), 0),
        (Digit("cf".to_string()), 1),
        (Digit("acdeg".to_string()), 2),
        (Digit("acdfg".to_string()), 3),
        (Digit("bcdf".to_string()), 4),
        (Digit("abdfg".to_string()), 5),
        (Digit("abdefg".to_string()), 6),
        (Digit("acf".to_string()), 7),
        (Digit("abcdefg".to_string()), 8),
        (Digit("abcdfg".to_string()), 9),
    ]);

    (0, part_2(&parsed_test_data, &real_digit_map) as u64)
}
