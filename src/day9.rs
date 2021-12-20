use std::num;

pub struct Coord {
    x: usize,
    y: usize,
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
    pub fn point_is_low_point(&self, coord: &Coord) -> bool {
        let point = self.0[coord.x as usize][coord.y as usize];

        let adjacent_coords = [
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: 0 },
            Direction { x: -1, y: 0 },
        ];

        adjacent_coords
            .iter()
            .filter(|adj| {
                ((adj.x + (coord.x as i64)) >= 0)
                    && ((adj.x + (coord.x as i64)) <= self.max_x())
                    && ((adj.y + (coord.y as i64)) >= 0)
                    && ((adj.y + (coord.y as i64)) <= self.max_y())
            })
            .all(|adj| {
                self.0[(adj.x + (coord.x as i64)) as usize][(adj.y + (coord.y as i64)) as usize]
                    > point
            })
    }
    fn max_x(&self) -> i64 {
        (self.0.len() - 1) as i64
    }
    fn max_y(&self) -> i64 {
        (self.0[0].len() - 1) as i64
    }

    pub fn find_low_points(&self) -> Vec<Coord> {
        let mut low_points = Vec::<Coord>::new();
        for x in 0..=self.max_x() {
            for y in 0..=self.max_y() {
                let coord = Coord {
                    x: x as usize,
                    y: y as usize,
                };
                if self.point_is_low_point(&coord) {
                    low_points.push(coord);
                }
            }
        }
        low_points
    }

    pub fn find_basin_size_for_low_point(&self, coord: &Coord) -> () {
        ()
    }

    pub fn find_low_point_risk_sum(&self) -> i64 {
        let mut low_point_risk_sum = 0;
        for low_point in self.find_low_points().iter() {
            low_point_risk_sum += self.0[low_point.x][low_point.y] + 1;
        }
        low_point_risk_sum
    }
}
fn parse_input_lines(raw_input_lines: &[String]) -> Result<HeightMap, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    HeightMap::new(&input_lines)
}

pub fn part_1(height_map: &HeightMap) -> i64 {
    // let cord = Coord { x: 1, y: 1 };
    // height_map.point_is_low_point(&cord);

    let cord = Coord { x: 2, y: 2 };
    height_map.point_is_low_point(&cord);

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
