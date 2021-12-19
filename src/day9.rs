use std::num;

pub struct Coord {
    x: i64,
    y: i64,
}

pub struct Direction {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
pub struct HeightMap(Vec<Vec<i64>>);
impl HeightMap {
    pub fn new(line: &Vec<&String>) -> Result<HeightMap, num::ParseIntError> {
        let line1 = line.clone();

        let height_map = line1
            .into_iter()
            .map(|row| {
                row.chars()
                    .map(|heights| heights.to_string().parse::<i64>())
                    .collect::<Result<Vec<i64>, num::ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<i64>>, num::ParseIntError>>()?;

        Ok(HeightMap(height_map))
    }
    pub fn point_is_low_point(&self, x: i64, y: i64) -> bool {
        let point = self.0[x as usize][y as usize];

        let adjacent_coords = [
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: 0 },
            Direction { x: -1, y: 0 },
        ];
        adjacent_coords
            .iter()
            .filter(|adj| {
                ((adj.x + x) >= 0)
                    && ((adj.x + x) <= self.max_x())
                    && ((adj.y + y) >= 0)
                    && ((adj.y + y) <= self.max_y())
            })
            .all(|adj| self.0[(adj.x + x) as usize][(adj.y + y) as usize] > point)
    }
    fn max_x(&self) -> i64 {
        (self.0.len() - 1) as i64
    }
    fn max_y(&self) -> i64 {
        (self.0[0].len() - 1) as i64
    }

    pub fn find_low_point_risk_sum(&self) -> i64 {
        let mut low_point_risk_sum = 0;
        for x in 0..=self.max_x() {
            for y in 0..=self.max_y() {
                if self.point_is_low_point(x as i64, y as i64) {
                    low_point_risk_sum += self.0[x as usize][y as usize] + 1;
                }
            }
        }
        low_point_risk_sum
    }
}
fn parse_input_lines(raw_input_lines: &[String]) -> Result<HeightMap, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    HeightMap::new(&input_lines)
}

pub fn part_1(height_map: &HeightMap) -> i64 {
    println!("test");

    height_map.find_low_point_risk_sum()
}

pub fn part_2(height_map: &HeightMap) -> i64 {
    0
}

pub fn day9(input_lines: &[String]) -> (u64, u64) {
    let height_map = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&height_map) as u64, part_2(&height_map) as u64)
}
