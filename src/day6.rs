use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fish(i64);
impl Fish {
    pub fn new(line: &str) -> Result<Fish, num::ParseIntError> {
        let value = line.parse::<i64>()?;
        Ok(Fish(value))
    }
    pub fn live_one_day(&mut self) -> Option<Fish> {
        if self.0 == 0 {
            self.0 = 6;
            Some(Fish(8))
        } else {
            self.0 -= 1;
            None
        }
    }
}

fn parse_input_lines(input_lines: &[String]) -> Result<Vec<Fish>, num::ParseIntError> {
    let mut parsed_data = Vec::new();

    let input_lines = input_lines[0].split(',').collect::<Vec<&str>>();

    for line in input_lines {
        parsed_data.push(Fish::new(line)?);
    }
    Ok(parsed_data)
}

pub fn fishes_population_after_x_days(
    fish: &mut Fish,
    days: i32,
    fish_lookups: &mut HashMap<FishKey, i64>,
) -> i64 {
    if days == 0 {
        return 1;
    }

    let key = FishKey {
        days,
        fish: Fish(fish.0),
    };

    if !fish_lookups.contains_key(&key) {
        let mut tot_fishes = 0;
        let fish_day_output = fish.live_one_day();

        if let Some(mut child_fish) = fish_day_output {
            tot_fishes += fishes_population_after_x_days(&mut child_fish, days - 1, fish_lookups);
        }
        tot_fishes += fishes_population_after_x_days(fish, days - 1, fish_lookups);

        fish_lookups.insert(key.clone(), tot_fishes);
    }
    return *fish_lookups.get(&key).unwrap();
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct FishKey {
    days: i32,
    fish: Fish,
}

pub fn predict_fishes_number(fishes: &mut Vec<Fish>, days: i32) -> i64 {
    let mut total_fishes = 0;

    let mut fish_lookups: HashMap<FishKey, i64> = HashMap::new();
    for fish in fishes.iter_mut() {
        total_fishes += fishes_population_after_x_days(fish, days, &mut fish_lookups);
    }
    total_fishes
}

pub fn part_1(parsed_data: &Vec<Fish>) -> i64 {
    let mut fishes = (*parsed_data).clone();

    predict_fishes_number(&mut fishes, 80)
}
pub fn part_2(parsed_data: &Vec<Fish>) -> i64 {
    let mut fishes = (*parsed_data).clone();

    predict_fishes_number(&mut fishes, 256)
}

pub fn day6(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1(&parsed_test_data) as u64,
        part_2(&parsed_test_data) as u64,
    )
}
