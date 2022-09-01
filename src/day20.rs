use std::collections::HashMap;
use std::num;
use std::os::windows::raw;

#[derive(Debug, Clone)]
pub struct Image {
    map: HashMap<Vec<isize>, char>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}
impl Image {
    pub fn new(input_lines: Vec<&String>) -> Image {
        let mut map = HashMap::new();

        let min_x = 0;
        let min_y = 0;

        let mut max_x = 0 as isize;
        let mut max_y = 0 as isize;

        for (x, row) in input_lines.into_iter().enumerate() {
            for (y, pixel) in row.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                map.insert(vec![x, y], pixel);

                // Update max values if needed
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }

        Image {
            map,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
    pub fn enhance(&mut self, enhance_algo: &String) {
        let old_map = self.map.clone();

        for x in (self.min_x - 1)..(self.max_x + 1) {
            for y in (self.min_y - 1)..(self.max_y + 1) {
                let coord = vec![x, y];
                if !self.map.contains_key(&coord) {
                    self.map.insert(coord.clone(), '.');
                }

                let algo_key = self.calc_algo_key(&coord);
                let new_value = enhance_algo.chars().nth(algo_key).unwrap();

                *self.map.get_mut(&coord).unwrap() = new_value;
            }
        }

        // for coord in old_map.keys() {
        //     let algo_key = self.calc_algo_key(coord);

        //     let new_value = enhance_algo.chars().nth(algo_key).unwrap();

        //     *self.map.get_mut(coord).unwrap() = new_value;
        // }
    }
    pub fn calc_algo_key(&mut self, coord: &Vec<isize>) -> usize {
        let directions = vec![
            vec![-1, -1],
            vec![-1, 0],
            vec![-1, 1],
            vec![0, -1],
            vec![0, 0],
            vec![0, 1],
            vec![1, -1],
            vec![1, 0],
            vec![1, 1],
        ];

        let mut binary_key = vec![];

        for direction in directions.iter() {
            let new_coord = vec![coord[0] + direction[0], coord[1] + direction[1]];

            // if !self.map.contains_key(&new_coord) {
            //     self.map.insert(new_coord.clone(), '.');
            // }

            // if #, add 1 to binary string, else at 0
            match self.map.get(&new_coord).unwrap() {
                '#' => binary_key.push('1'),
                '.' => binary_key.push('0'),
                _ => panic!("invalid value for pixel!"),
            }
        }
        // println!("binary key vec is {:?}", binary_key);
        let binary_key = binary_key.into_iter().collect::<String>();
        // println!("binary string vec is {:?}", binary_key);
        usize::from_str_radix(&binary_key, 2).unwrap()
    }

    pub fn count_lit_pixles(&self) -> usize {
        self.map
            .keys()
            .filter(|x| *self.map.get(*x).unwrap() == '#')
            .count()
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<(Image, String), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let mut input_lines = input_lines.clone();

    let enhance_algo = input_lines.remove(0).clone();
    input_lines.remove(0);

    let image = Image::new(input_lines);

    Ok((image, enhance_algo))
}

pub fn part_1((image, enhance_algo): (&Image, &String)) -> i64 {
    let a = 1;

    let mut image = image.clone();

    for _ in 0..2 {
        image.enhance(enhance_algo);
    }

    // println!("image at 1,1 is: {}", image.map.get(&vec![1, 1]).unwrap());
    // println!("here");
    image.count_lit_pixles() as i64
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
