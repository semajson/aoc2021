use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputLine {
    encoded_digits: Vec<Digit>,
    output: Vec<Digit>,
}
impl InputLine {
    pub fn new(line: &str) -> Result<InputLine, &'static str> {
        let input = line.split("|").collect::<Vec<&str>>();

        if input.len() != 2 {
            return Err("Error: input line doesn't contain 2 things after parsing");
        }
        let encoded_digits = input[0]
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

        Ok(InputLine {
            encoded_digits,
            output,
        })
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
    println!("the count is {}", count);

    count
}

fn get_only_unique_values(values: &Vec<usize>) -> Vec<usize> {
    values
        .iter()
        .filter(|x| values.iter().filter(|y| y == x).count() == 1)
        .map(|x| *x)
        .collect::<Vec<usize>>()
}

fn find_output(InputLine: &InputLine, real_digit_map: &HashMap<Digit, i32>) -> i64 {
    // setup possible actual char for each encoded char
    let possible_actual_chars = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
        "g".to_string(),
    ];
    let mut encoded_char_to_actual_char = HashMap::from([
        ("a".to_string(), possible_actual_chars.clone()),
        ("b".to_string(), possible_actual_chars.clone()),
        ("c".to_string(), possible_actual_chars.clone()),
        ("d".to_string(), possible_actual_chars.clone()),
        ("e".to_string(), possible_actual_chars.clone()),
        ("f".to_string(), possible_actual_chars.clone()),
        ("g".to_string(), possible_actual_chars.clone()),
    ]);

    // Do initial trim of possible chars based on
    // the known digits. Known digits can be found by using the num of chars they contain.

    // Find the unique lengths
    let real_digit_lens = real_digit_map
        .iter()
        .map(|z| z.0 .0.len())
        .collect::<Vec<usize>>();

    let real_digit_lens_only_once = get_only_unique_values(&real_digit_lens);

    for encoded_digit in InputLine.encoded_digits.iter() {
        if real_digit_lens_only_once.contains(&encoded_digit.0.len()) {
            let mut actual_digit = None;
            for (k, _) in real_digit_map {
                if k.0.len() == encoded_digit.0.len() {
                    actual_digit = Some(k);
                }
            }

            reduce_possible_chars_on_known_digits(
                &encoded_digit,
                actual_digit.unwrap(),
                &mut encoded_char_to_actual_char,
            )
        }
    }

    // Now reduce based on the known chars - some chars appear a unique amount
    // of times, that can be leveraged here.
    let mut actual_char_counts: HashMap<String, usize> = HashMap::from([
        ("a".to_string(), 0),
        ("b".to_string(), 0),
        ("c".to_string(), 0),
        ("d".to_string(), 0),
        ("e".to_string(), 0),
        ("f".to_string(), 0),
        ("g".to_string(), 0),
    ]);

    for (digit, _) in real_digit_map.iter() {
        for (char, count) in actual_char_counts.iter_mut() {
            if digit.0.contains(char) {
                *count += 1;
            }
        }
    }

    let unique_actual_char_counts = get_only_unique_values(
        &actual_char_counts
            .values()
            .map(|x| *x as usize)
            .collect::<Vec<usize>>(),
    );

    for encoded_digit_iter in vec!["a", "b", "c", "d", "e", "f", "g"] {
        let mut encoded_count = 0;
        for encoded_digit in InputLine.encoded_digits.iter() {
            if encoded_digit.0.contains(encoded_digit_iter) {
                encoded_count += 1;
            }
        }
        if unique_actual_char_counts.contains(&encoded_count) {
            for (actual_char, count) in actual_char_counts.iter() {
                if *count == encoded_count {
                    let mut possible_actual_chars = encoded_char_to_actual_char
                        .get_mut(&encoded_digit_iter.to_string() as &str)
                        .unwrap();
                    assert!(possible_actual_chars.contains(&actual_char));

                    *possible_actual_chars = vec![actual_char.clone()];
                    break;
                }
            }
        }
    }

    // Now sudoku the rest
    while !solved_vals(&encoded_char_to_actual_char) {
        let mut new_encoded_char_to_actual_char = encoded_char_to_actual_char.clone();

        for (encoded_char, actual_chars) in encoded_char_to_actual_char.iter_mut() {
            if actual_chars.len() == 1 {
                // ensure no other possible values have this

                for other_encoded_char in vec!["a", "b", "c", "d", "e", "f", "g"] {
                    if encoded_char != other_encoded_char {
                        let mut possible_actual_chars_1 = new_encoded_char_to_actual_char
                            .get_mut(&other_encoded_char as &str)
                            .unwrap();

                        possible_actual_chars_1.retain(|x| *x != actual_chars[0]);
                    }
                }
                // break;
            }
        }
        encoded_char_to_actual_char = new_encoded_char_to_actual_char;
    }

    return decode_digit(
        &encoded_char_to_actual_char,
        &InputLine.output[0],
        real_digit_map,
    ) * 1000
        + decode_digit(
            &encoded_char_to_actual_char,
            &InputLine.output[1],
            real_digit_map,
        ) * 100
        + decode_digit(
            &encoded_char_to_actual_char,
            &InputLine.output[2],
            real_digit_map,
        ) * 10
        + decode_digit(
            &encoded_char_to_actual_char,
            &InputLine.output[3],
            real_digit_map,
        ) * 1;
}

fn decode_digit(
    encoded_char_to_actual_char: &HashMap<String, Vec<String>>,
    digit: &Digit,
    real_digit_map: &HashMap<Digit, i32>,
) -> i64 {
    let mut new_digit = "".to_string();
    for char in digit.0.chars() {
        let new_char = encoded_char_to_actual_char.get(&char.to_string()).unwrap()[0].clone();
        new_digit.push(new_char.chars().nth(0).unwrap());
    }

    // order string
    let mut new_digit = new_digit.chars().collect::<Vec<char>>();
    new_digit.sort();
    let new_digit = new_digit.iter().collect::<String>();
    // .iter()
    // .collect::<String>();

    *(real_digit_map.get(&Digit(new_digit)).unwrap()) as i64
}

fn reduce_possible_chars_on_known_digits(
    encoded_digit: &Digit,
    actual_digit: &Digit,
    encoded_char_to_actual_char: &mut HashMap<String, Vec<String>>,
) -> () {
    assert!(encoded_digit.0.len() == actual_digit.0.len());
    for (_, encoded_char) in encoded_digit.0.chars().enumerate() {
        let mut possible_actual_chars = encoded_char_to_actual_char
            .get_mut(&encoded_char.to_string() as &str)
            .unwrap();
        possible_actual_chars.retain(|x| actual_digit.0.contains(x));
    }
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

    (
        part_1(&parsed_test_data, &real_digit_map) as u64,
        part_2(&parsed_test_data, &real_digit_map) as u64,
    )
}
