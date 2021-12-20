use std::collections::HashSet;
use std::num;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        let line = line.clone();

        let height_map = line
            .into_iter()
            .map(|row| {
                row.chars()
                    .map(|heights| heights.to_string().parse::<i64>())
                    .collect::<Result<Vec<i64>, num::ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<i64>>, num::ParseIntError>>()?;

        Ok(HeightMap(height_map))
    }

    fn adjacent_points(&self, coord: &Coord) -> Vec<Coord> {
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
            .map(|adj| Coord {
                x: (adj.x + (coord.x as i64)) as usize,
                y: (adj.y + (coord.y as i64)) as usize,
            })
            .collect::<Vec<Coord>>()
    }

    pub fn point_is_low_point(&self, coord: &Coord) -> bool {
        let point = self.0[coord.x][coord.y];

        self.adjacent_points(&coord)
            .iter()
            .all(|adj| self.0[adj.x][adj.y] > point)
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

    pub fn find_basin_size_for_low_point(&self, coord: &Coord) -> usize {
        let coord = coord.clone();

        assert!(self.point_is_low_point(&coord));

        // Find the basin via expanding through each perimeter
        let mut seen_coords = HashSet::new();
        let mut current_perimeter = vec![coord];

        while !current_perimeter.is_empty() {
            let mut new_perimeter = vec![];

            for perimeter_coord in current_perimeter {
                let new_points = self
                    .adjacent_points(&perimeter_coord)
                    .into_iter()
                    .filter(|adj_coord| {
                        (!seen_coords.contains(adj_coord)) && (self.0[adj_coord.x][adj_coord.y] < 9)
                    })
                    .collect::<Vec<Coord>>();

                for point in new_points.into_iter() {
                    seen_coords.insert(point.clone());
                    new_perimeter.push(point);
                }
            }

            current_perimeter = new_perimeter;
        }

        seen_coords.len()
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
    let cord = Coord { x: 2, y: 2 };
    height_map.point_is_low_point(&cord);

    height_map.find_low_point_risk_sum()
}

pub fn part_2(height_map: &HeightMap) -> i64 {
    let mut sizes = height_map
        .find_low_points()
        .iter()
        .map(|point| height_map.find_basin_size_for_low_point(point))
        .collect::<Vec<_>>();
    println!("d");
    sizes.sort_by(|a, b| b.cmp(a));

    (sizes[0] * sizes[1] * sizes[2]) as i64
}

pub fn day9(input_lines: &[String]) -> (u64, u64) {
    let height_map = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&height_map) as u64, part_2(&height_map) as u64)
}
