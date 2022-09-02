use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone)]
pub struct Image {
    map: HashMap<Vec<isize>, char>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    infinite_pixel_state: char,
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
            infinite_pixel_state: '.',
        }
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        let mut debug = vec![];

        for x in (self.min_x)..=(self.max_x) {
            let mut row = vec![];
            for y in (self.min_y)..=(self.max_y) {
                match self.map.get(&vec![x, y]).unwrap() {
                    '#' => row.push('#'),
                    '.' => row.push('.'),
                    _ => panic!("invalid value for pixel!"),
                }
            }
            println!("{:?}", row.iter().collect::<String>());
            debug.push(row);
        }
    }

    pub fn enhance(&mut self, enhance_algo: &String) {
        let old_map = self.map.clone();

        // grow the boarders
        self.max_x += 1;
        self.min_x -= 1;

        self.max_y += 1;
        self.min_y -= 1;

        for x in (self.min_x)..=(self.max_x) {
            for y in (self.min_y)..=(self.max_y) {
                let coord = vec![x, y];
                if !self.map.contains_key(&coord) {
                    self.map
                        .insert(coord.clone(), self.infinite_pixel_state.clone());
                }

                let algo_key = self.calc_algo_key(&coord, &old_map);
                let new_value = enhance_algo.chars().nth(algo_key).unwrap();

                *self.map.get_mut(&coord).unwrap() = new_value;
            }
        }

        // toggle infinite pixel state if required
        if (enhance_algo.chars().nth(0).unwrap() == '#')
            && (enhance_algo.chars().last().unwrap() == '.')
        {
            match self.infinite_pixel_state {
                '.' => self.infinite_pixel_state = '#',
                '#' => self.infinite_pixel_state = '.',
                _ => panic!("invalid value for pixel!"),
            }
        }
    }

    pub fn calc_algo_key(&self, coord: &Vec<isize>, old_map: &HashMap<Vec<isize>, char>) -> usize {
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

            let mut pixel_value = self.infinite_pixel_state.clone();
            if old_map.contains_key(&new_coord) {
                pixel_value = *old_map.get(&new_coord).unwrap();
            }

            // if #, add 1 to binary string, else at 0
            match pixel_value {
                '#' => binary_key.push('1'),
                '.' => binary_key.push('0'),
                _ => panic!("invalid value for pixel!"),
            }
        }
        let binary_key = binary_key.into_iter().collect::<String>();
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
    let mut image = image.clone();

    image.enhance(enhance_algo);
    image.enhance(enhance_algo);

    image.count_lit_pixles() as i64
}

pub fn part_2((image, enhancement_algorithm): (&Image, &String)) -> i64 {
    let mut image = image.clone();

    for _ in 0..50 {
        image.enhance(enhancement_algorithm);
    }

    image.count_lit_pixles() as i64
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
