use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InputLine {
    encoded_digits: Vec<Digit>,
    output_digits: Vec<Digit>,
}
impl InputLine {
    pub fn new(line: &str) -> Result<InputLine, &'static str> {
        let input = line.split('|').collect::<Vec<_>>();

        if input.len() != 2 {
            return Err("Error: input line doesn't contain 2 things after parsing");
        }
        let encoded_digits = input[0]
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| Digit(x.to_string()))
            .collect::<Vec<Digit>>();

        let output_digits = input[1]
            .split(' ')
            .collect::<Vec<_>>()
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| Digit(x.to_string()))
            .collect::<Vec<Digit>>();

        Ok(InputLine {
            encoded_digits,
            output_digits,
        })
    }
}
fn parse_input_lines(input_lines: &[String]) -> Result<Vec<InputLine>, &'static str> {
    let mut parsed_data = Vec::new();

    for line in input_lines.iter() {
        parsed_data.push(InputLine::new(line)?);
    }
    Ok(parsed_data)
}

pub fn part_1(
    parsed_data: &Vec<InputLine>,
    actual_chars_to_digit_map: &HashMap<Digit, i32>,
) -> i64 {
    let real_digit_lens = actual_chars_to_digit_map
        .iter()
        .map(|z| z.0 .0.len())
        .collect::<Vec<usize>>();

    let unique_digit_lens = get_only_unique_values(&real_digit_lens);

    let mut count_knowns = 0;
    for line in parsed_data.iter() {
        for digit in line.output_digits.iter() {
            if unique_digit_lens.contains(&digit.0.len()) {
                count_knowns += 1;
            }
        }
    }

    count_knowns
}

fn get_only_unique_values(values: &[usize]) -> Vec<usize> {
    values
        .iter()
        .filter(|x| values.iter().filter(|y| y == x).count() == 1)
        .copied()
        .collect::<Vec<usize>>()
}

pub fn part_2(
    parsed_data: &Vec<InputLine>,
    actual_chars_to_digit_map: &HashMap<Digit, i32>,
    all_chars: &[&str],
) -> i64 {
    let mut output_count = 0;

    for line in parsed_data.iter() {
        output_count += find_output(line, actual_chars_to_digit_map, all_chars);
    }
    output_count
}

fn find_output(
    input_line: &InputLine,
    actual_chars_to_digit_map: &HashMap<Digit, i32>,
    all_chars: &[&str],
) -> i64 {
    // Setup possible actual chars for each encoded char
    let all_possible_actual_chars = all_chars.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let mut encoded_char_to_actual_char = all_chars
        .iter()
        .map(|x| (x.to_string(), all_possible_actual_chars.clone()))
        .collect::<HashMap<String, Vec<String>>>();

    // Do initial trim of possible actual chars based on the known digits.
    // Known digits can be found by using the num of chars they contain.
    reduce_using_known_digits(
        input_line,
        &mut encoded_char_to_actual_char,
        actual_chars_to_digit_map,
    );

    // Now reduce based on the known chars - some chars appear a unique amount
    // of times when summed over all 10 digits, so we can directly map from
    // encoded char -> actual char.
    reduce_using_known_chars(
        input_line,
        &mut encoded_char_to_actual_char,
        actual_chars_to_digit_map,
        all_chars,
    );

    // Now sudoku the rest
    solve_sudoku(&mut encoded_char_to_actual_char, all_chars);

    let mut output = 0;
    for (i, output_digit) in input_line.output_digits.iter().enumerate() {
        let power = (input_line.output_digits.len() - (i + 1)) as i64;
        let multiplier = i64::pow(10, power as u32);
        output += decode_digit(
            &encoded_char_to_actual_char,
            output_digit,
            actual_chars_to_digit_map,
        ) * multiplier;
    }
    output
}

fn reduce_using_known_digits(
    input_line: &InputLine,
    encoded_char_to_actual_char: &mut HashMap<String, Vec<String>>,
    actual_chars_to_digit_map: &HashMap<Digit, i32>,
) {
    // Find the unique lengths
    let real_digit_lens = actual_chars_to_digit_map
        .iter()
        .map(|z| z.0 .0.len())
        .collect::<Vec<usize>>();
    let real_digit_lens_only_once = get_only_unique_values(&real_digit_lens);

    // Now reduce
    for encoded_digit in input_line.encoded_digits.iter() {
        if real_digit_lens_only_once.contains(&encoded_digit.0.len()) {
            let mut actual_digit = None;
            for k in actual_chars_to_digit_map.keys() {
                if k.0.len() == encoded_digit.0.len() {
                    actual_digit = Some(k);
                }
            }
            reduce_using_known_digit(
                encoded_digit,
                actual_digit.unwrap(),
                encoded_char_to_actual_char,
            )
        }
    }
}

fn reduce_using_known_digit(
    encoded_digit: &Digit,
    actual_digit: &Digit,
    encoded_char_to_actual_char: &mut HashMap<String, Vec<String>>,
) {
    assert!(encoded_digit.0.len() == actual_digit.0.len());

    for (_, encoded_char) in encoded_digit.0.chars().enumerate() {
        let possible_actual_chars = encoded_char_to_actual_char
            .get_mut(&encoded_char.to_string() as &str)
            .unwrap();
        possible_actual_chars.retain(|x| actual_digit.0.contains(x));
    }
}

fn reduce_using_known_chars(
    input_line: &InputLine,
    encoded_char_to_actual_char: &mut HashMap<String, Vec<String>>,
    actual_chars_to_digit_map: &HashMap<Digit, i32>,
    all_chars: &[&str],
) {
    // Get the unique char counts across all digits
    let mut actual_char_counts = HashMap::new();
    for actual_char in all_chars {
        actual_char_counts.insert(actual_char.to_string(), 0);
    }
    for (digit, _) in actual_chars_to_digit_map.iter() {
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

    // Now reduce
    for char_ in all_chars {
        let mut encoded_count = 0;
        for encoded_digit in input_line.encoded_digits.iter() {
            if encoded_digit.0.contains(char_) {
                encoded_count += 1;
            }
        }
        if unique_actual_char_counts.contains(&encoded_count) {
            for (actual_char, count) in actual_char_counts.iter() {
                if *count == encoded_count {
                    let possible_actual_chars = encoded_char_to_actual_char
                        .get_mut(&char_.to_string() as &str)
                        .unwrap();
                    assert!(possible_actual_chars.contains(actual_char));

                    *possible_actual_chars = vec![actual_char.clone()];
                    break;
                }
            }
        }
    }
}

fn solve_sudoku(
    encoded_char_to_actual_char: &mut HashMap<String, Vec<String>>,
    all_chars: &[&str],
) {
    while !solved_sudoku(encoded_char_to_actual_char) {
        let mut new_encoded_char_to_actual_char = encoded_char_to_actual_char.clone();

        for (encoded_char, possible_actual_chars) in encoded_char_to_actual_char.iter_mut() {
            if possible_actual_chars.len() == 1 {
                // We now know the actual char here.
                // Ensure this isn't an option for any other encoded char
                for other_encoded_char in all_chars {
                    if encoded_char != other_encoded_char {
                        let other_possible_actual_chars = new_encoded_char_to_actual_char
                            .get_mut(other_encoded_char as &str)
                            .unwrap();

                        other_possible_actual_chars.retain(|x| *x != possible_actual_chars[0]);
                    }
                }
            }
        }
        *encoded_char_to_actual_char = new_encoded_char_to_actual_char;
    }
}

fn solved_sudoku(encoded_value_to_possible_values: &HashMap<String, Vec<String>>) -> bool {
    encoded_value_to_possible_values
        .values()
        .all(|val| val.len() == 1)
}

fn decode_digit(
    encoded_char_to_actual_char: &HashMap<String, Vec<String>>,
    encoded_digit: &Digit,
    actual_chars_to_digit_map: &HashMap<Digit, i32>,
) -> i64 {
    let mut actual_digit = "".to_string();

    // Swap encoded chars for actual chars
    for encoded_char in encoded_digit.0.chars() {
        let actual_char = encoded_char_to_actual_char
            .get(&encoded_char.to_string())
            .unwrap()[0]
            .clone();
        actual_digit.push(actual_char.chars().next().unwrap());
    }

    // Order the chars (required for lookup)
    let mut actual_digit = actual_digit.chars().collect::<Vec<char>>();
    actual_digit.sort_unstable();
    let actual_digit = actual_digit.iter().collect::<String>();

    *(actual_chars_to_digit_map.get(&Digit(actual_digit)).unwrap()) as i64
}

pub fn day8(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    let actual_chars_to_digit_map: HashMap<Digit, i32> = HashMap::from([
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
    let all_chars = ["a", "b", "c", "d", "e", "f", "g"];
    (
        part_1(&parsed_test_data, &actual_chars_to_digit_map) as u64,
        part_2(&parsed_test_data, &actual_chars_to_digit_map, &all_chars) as u64,
    )
}
