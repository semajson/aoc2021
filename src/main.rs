// test with https://adventofcode.com/2018/day/1
use std::env;
mod day1;

type DayFunction = fn(&[String]) -> (u64, u64);
static DAY_FUNCTIONS: [DayFunction; 1] = [
    day1::day1,
    // day2::day2,
    // day3::day3,

];

// pub fn build_filename(day: i32, suffix: str) ->

pub fn parse_test_input_data(day: i32) -> &[String] {
    let filename = "inputs/day ".to_owned();
    filename = filename + day +  "_test" + ".txt";
    parse_data_from_file(filename).unwrap_or_else( panic!("failed read test data{:?}", filename));

}

pub fn parse_real_input_data(day: i32) -> &[String] {
    let filename = "inputs/day ".to_owned();
    filename = filename + day + "_real" + ".txt"
    parse_data_from_file(filename).unwrap_or_else( panic!("failed read real data{:?}", filename));

}

fn parse_data_from_file(filename: &str) -> Result<&[String], io::Error> {
    let raw_input = fs::read_to_string(filename)?;
    raw_input
}

fn solve_day( day: i32) {
    DAY_FUNCTIONS[day - 1];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize;
    if (args.len() != 1) {
        panic!("Incorrect number of days specified")
    };
    let day =  args[1].parse::<usize>().expect("Please provide the day number as an integer.");
    solve_day(day);
}