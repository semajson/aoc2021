use std::collections::HashMap;
use std::num;
use std::os::windows::raw;

#[derive(Debug, Clone)]
pub struct Image {
    map: HashMap<Vec<isize>, char>,
    //     min_x: usize,
    //     min_y: usize,
    //     max_x: usize,
    //     max_y: usize,
}
impl Image {
    pub fn new(input_lines: Vec<&String>) -> Image {
        let mut map = HashMap::new();

        // let min_x = 0;
        // let min_y = 0;

        // let mut max_x = 0;
        // let mut max_y = 0;

        for (x, row) in input_lines.into_iter().enumerate() {
            for (y, pixel) in row.chars().enumerate() {
                map.insert(vec![x as isize, y as isize], pixel);

                // // Update max values if needed
                // if x > max_x {
                //     max_x = x;
                // }
                // if y > max_y {
                //     max_y = y;
                // }
            }
        }

        Image {
            map,
            // min_x,
            // min_y,
            // max_x,
            // max_y,
        }
    }
    pub fn enhance(&mut self, enhance_algo: &String) {
        let old_map = self.map.clone();

        // for x in (self.min_x - 1)..(self.max_x + 1) {
        //     for y in (self.min_y - 1)..(self.max_y + 1) {
        //         if !self.map.contains_key(&vec![x, y]) {
        //             self.map.insert(vec![x, y], '.');
        //         }

        //         let algo_key = self.calc_algo_key(x, y);
        //         let new_value = enhance_algo.chars().nth(algo_key).unwrap();

        //         *self.map.get_mut(&vec![x, y]).unwrap() = new_value;
        //     }
        // }

        for coord in old_map.keys() {
            let algo_key = self.calc_algo_key(coord);

            let new_value = enhance_algo.chars().nth(algo_key).unwrap();

            *self.map.get_mut(coord).unwrap() = new_value;
        }
    }
    pub fn calc_algo_key(&mut self, coord: &Vec<isize>) -> usize {
        // let binary_key = vec![];

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

        for direction in directions.iter() {
            let new_coord = vec![coord[0] + direction[0], coord[1] + direction[1]];

            if !self.map.contains_key(&new_coord) {
                self.map.insert(new_coord, '.');
            }

            // if #, add 1 to binary string, else at 0
        }

        // note, deal with error handling here
        0 as usize
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<(Image, String), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let mut input_lines = input_lines.clone();

    let enhance_algo = input_lines.remove(0).clone();

    let image = Image::new(input_lines);

    Ok((image, enhance_algo))
}

pub fn part_1((image, enhance_algo): (&Image, &String)) -> i64 {
    let a = 1;

    let mut image = image.clone();

    for _ in 0..30 {
        image.enhance(enhance_algo);
    }

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
