// test with https://adventofcode.com/2018/day/1
fn main() {
    println!("Hello, world!");
    let mut parsed_test_data = practice::parse_test_input_data().unwrap();
    practice::part_1(&parsed_test_data);
    practice::part_2(&parsed_test_data);

    let mut parsed_real_data = practice::parse_real_input_data().unwrap();
    practice::part_1(&parsed_real_data);
    practice::part_2(&parsed_real_data);
}
