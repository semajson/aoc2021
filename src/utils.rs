use std::io;
use std::fs;

pub fn parse_test_input_data(day: i32) -> Vec<String>  {
    let filename = build_filename(day, "_test");

    parse_data_from_file(&filename).unwrap_or_else(|err| {
        panic!("Got error {} when reading the test data  file {}", err, filename);})

}

pub fn parse_real_input_data(day: i32) -> Vec<String> {
    let filename = build_filename(day, "_real");

    parse_data_from_file(&filename).unwrap_or_else(|err| {
        panic!("Got error {} when reading the real data file {}", err, filename);})
}

fn build_filename(day: i32, suffix: &str) -> String {
    let prefix = "inputs/day".to_owned();
    prefix + &day.to_string() +  suffix + ".txt"

}

fn parse_data_from_file(filename: &str) -> Result<Vec<String> , io::Error> {
    let raw_input = fs::read_to_string(filename)?;
    Ok(raw_input.lines().map(std::string::ToString::to_string).collect())
}