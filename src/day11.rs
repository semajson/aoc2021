use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Octopus(usize);

#[derive(Debug, Clone)]
pub struct Grid {
    octopuses: Vec<Vec<usize>>,
    octopuses_flashed_this_step: Vec<Coord>,
}
impl Grid {
    pub fn new(input_lines: Vec<&String>) -> Result<Grid, num::ParseIntError> {
        let input_lines = input_lines.to_owned();

        let octopuses = input_lines
            .iter()
            .map(|x| {
                x.chars()
                    .map(|x| x.to_string().parse::<usize>())
                    .collect::<Result<Vec<usize>, num::ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<usize>>, num::ParseIntError>>();
        let octopuses = octopuses?;

        // let octopuses = octopuses
        //     .iter()
        //     .map(|x| {
        //         x.iter()
        //             .map(|y| Octopus(y.clone()))
        //             .collect::<Vec<Octopus>>()
        //     })
        //     .collect::<Vec<Vec<Octopus>>>();

        Ok(Grid {
            octopuses: octopuses,
            octopuses_flashed_this_step: vec![],
        })
    }

    pub fn do_step(&mut self) {
        self.increase_all_octopuses_1();
    }
    fn increase_all_octopuses_1(&mut self) {
        for row in self.octopuses.iter_mut() {
            for octopus in row.iter_mut() {
                *octopus += 1;
            }
        }
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Grid, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    Grid::new(input_lines)
}

pub fn part_1(grid: &Grid) -> i64 {
    println!("test");
    let mut grid = grid.clone();
    grid.do_step();

    grid.do_step();
    grid.do_step();
    grid.do_step();

    0
}

pub fn part_2(parsed_data: &Grid) -> i64 {
    0
}

pub fn day11(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&parsed_data) as u64, part_2(&parsed_data) as u64)
}
