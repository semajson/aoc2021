use std::collections::HashMap;
use std::num;
use std::os::windows::raw;

#[derive(Debug, Clone)]
pub struct Image {
    map: HashMap<Vec<usize>, char>,
}
impl Image {
    pub fn new(input_lines: Vec<&String>) -> Image {
        let mut map = HashMap::new();

        for (x, row) in input_lines.into_iter().enumerate() {
            for (y, pixel) in row.chars().enumerate() {
                map.insert(vec![x, y], pixel);
            }
        }

        Image { map }
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<(Image, String), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let mut input_lines = input_lines.clone();

    let enhance_algo = input_lines.remove(0).clone();

    let image = Image::new(input_lines);

    Ok((image, enhance_algo))
}

pub fn part_1((image, enhancement_algorithm): (&Image, &String)) -> i64 {
    let a = 1;

    let image = image.clone();

    println!("image at 1,1 is: {}", image.map.get(&vec![1, 1]).unwrap());
    println!("here");
    2
}

pub fn part_2((image, enhancement_algorithm): (&Image, &String)) -> i64 {
    1
}

pub fn day20(input_lines: &[String]) -> (u64, u64) {
    let (image, algo) = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1((&image, &algo)) as u64,
        part_2((&image, &algo)) as u64,
    )
}
