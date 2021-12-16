// test with https://adventofcode.com/2018/day/1
use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

mod utils;

type DayFunction = fn(&[String]) -> (u64, u64);
static DAY_FUNCTIONS: [DayFunction; 7] = [
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    day7::day7,
];

fn solve_day(day: usize) {
    let raw_input = utils::parse_test_input_data(day as i32);
    let (part1, part2) = DAY_FUNCTIONS[day - 1](&raw_input);
    println!("part 1 answer is: {}", part1);
    println!("part 2 answer is: {}", part2);

    let raw_input = utils::parse_real_input_data(day as i32);
    let (part1, part2) = DAY_FUNCTIONS[day - 1](&raw_input);
    println!("part 1 answer is: {}", part1);
    println!("part 2 answer is: {}", part2);
}

#[test]
fn day_test() {
    solve_day(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Incorrect number of days specified")
    };
    let day = args[1]
        .parse::<usize>()
        .expect("Please provide the day number as an integer.");
    solve_day(day);
}
