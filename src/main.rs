// test with https://adventofcode.com/2018/day/1
use std::env;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod utils;

type DayFunction = fn(&[String]) -> (u64, u64);
static DAY_FUNCTIONS: [DayFunction; 20] = [
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    day7::day7,
    day8::day8,
    day9::day9,
    day10::day10,
    day11::day11,
    day12::day12,
    day13::day13,
    day14::day14,
    day15::day15,
    day16::day16,
    day17::day17,
    day18::day18,
    day19::day19,
    day20::day20,
];

fn solve_day(day: usize) {
    let raw_input = utils::parse_test_input_data(day as i32);
    let (part1, part2) = DAY_FUNCTIONS[day - 1](&raw_input);
    println!("part 1 answer is: {}", part1);
    println!("part 2 answer is: {}", part2);

    // let raw_input = utils::parse_real_input_data(day as i32);
    // let (part1, part2) = DAY_FUNCTIONS[day - 1](&raw_input);
    // println!("part 1 answer is: {}", part1);
    // println!("part 2 answer is: {}", part2);
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
