use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::num;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Octopus {
    x: usize,
    y: usize,
}
impl fmt::Display for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

pub struct Direction {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
pub struct Grid {
    octopuses_energies: HashMap<Octopus, usize>,
    octopuses_flashed_this_step: Vec<Octopus>,
    max_x: i64,
    max_y: i64,
    flashes: i64,
    all_flashed: bool,
}
impl Grid {
    pub fn new(input_lines: Vec<&String>) -> Result<Grid, num::ParseIntError> {
        let input_lines = input_lines.to_owned();

        let octopuses_energies_vec = input_lines
            .iter()
            .map(|x| {
                x.chars()
                    .map(|x| x.to_string().parse::<usize>())
                    .collect::<Result<Vec<usize>, num::ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<usize>>, num::ParseIntError>>();
        let octopuses_energies_vec = octopuses_energies_vec?;

        let mut octopuses_energies = HashMap::new();

        let max_x = octopuses_energies_vec.len() - 1;
        let max_y = octopuses_energies_vec[0].len() - 1;

        for x in 0..=max_x {
            for y in 0..=max_y {
                octopuses_energies.insert(Octopus { x, y }, octopuses_energies_vec[x][y]);
            }
        }

        Ok(Grid {
            octopuses_energies: octopuses_energies,
            octopuses_flashed_this_step: vec![],
            max_x: max_x as i64,
            max_y: max_y as i64,
            flashes: 0,
            all_flashed: false,
        })
    }

    pub fn do_step(&mut self) {
        self.increase_all_octopuses_1();

        // Deal with flash
        let mut already_flashed_octopuses = HashSet::new();
        let mut flashing_octopuses = self.get_first_octopus_to_flash_in_step();
        for flashing_octopus in flashing_octopuses.iter() {
            already_flashed_octopuses.insert(flashing_octopus.clone());
        }

        while !flashing_octopuses.is_empty() {
            let mut new_flashing_octopuses = vec![];

            for flashing_octopus in flashing_octopuses.iter() {
                // Deal with flashed octopus
                let flashing_octopus_energy =
                    self.octopuses_energies.get_mut(&flashing_octopus).unwrap();
                assert!(*flashing_octopus_energy == 10);
                *flashing_octopus_energy = 0;

                // Deal with surrounding octopuses
                let surrounding_octopuses = self.get_surrounding_octopuses(flashing_octopus);
                for surrounding_octopus in surrounding_octopuses.into_iter() {
                    if !already_flashed_octopuses.contains(&surrounding_octopus) {
                        // println!("{}", surrounding_octopus);
                        let energy = self
                            .octopuses_energies
                            .get_mut(&surrounding_octopus)
                            .unwrap();
                        *energy += 1;
                        if *energy > 9 {
                            already_flashed_octopuses.insert(surrounding_octopus.clone());
                            new_flashing_octopuses.push(surrounding_octopus);
                        }
                    }
                }

                // update grid data
                self.flashes += 1;
            }

            flashing_octopuses = new_flashing_octopuses;
        }
        if already_flashed_octopuses.len() == ((self.max_x + 1) * (self.max_y + 1)) as usize {
            self.all_flashed = true;
        }
    }

    fn increase_all_octopuses_1(&mut self) {
        for energy in self.octopuses_energies.values_mut() {
            (*energy) += 1;
        }
    }

    fn get_first_octopus_to_flash_in_step(&self) -> Vec<Octopus> {
        let mut flashing_octopuses = vec![];
        for (octopus, energy) in self.octopuses_energies.iter() {
            if *energy > 9 {
                flashing_octopuses.push(octopus.clone());
            }
        }
        flashing_octopuses
    }

    fn get_surrounding_octopuses(&self, octopus: &Octopus) -> Vec<Octopus> {
        let directions = [
            Direction { x: 0, y: 1 },
            Direction { x: 0, y: -1 },
            Direction { x: 1, y: -1 },
            Direction { x: 1, y: 0 },
            Direction { x: 1, y: 1 },
            Direction { x: -1, y: -1 },
            Direction { x: -1, y: 0 },
            Direction { x: -1, y: 1 },
        ];

        let surrounding_octopuses = directions
            .iter()
            .filter(|adj| {
                ((adj.x + (octopus.x as i64)) >= 0)
                    && ((adj.x + (octopus.x as i64)) <= self.max_x)
                    && ((adj.y + (octopus.y as i64)) >= 0)
                    && ((adj.y + (octopus.y as i64)) <= self.max_y)
            })
            .map(|adj| Octopus {
                x: (adj.x + (octopus.x as i64)) as usize,
                y: (adj.y + (octopus.y as i64)) as usize,
            })
            .collect::<Vec<Octopus>>();

        surrounding_octopuses
    }

    // fn debug_grid(&self) -> Vec<Vec<usize>> {
    //     let mut debug_view = vec![vec![0; (self.max_y + 1) as usize]; (self.max_x + 1) as usize];
    //     for (octopus, energy) in self.octopuses.iter() {
    //         debug_view[octopus.x][octopus.y] = *energy;
    //     }
    //     debug_view
    // }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Grid, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    Grid::new(input_lines)
}

pub fn part_1(grid: &Grid) -> i64 {
    let mut grid = grid.clone();

    for _ in 0..100 {
        grid.do_step();
    }

    grid.flashes
}

pub fn part_2(grid: &Grid) -> i64 {
    let mut grid = grid.clone();

    let mut step_num = 0;
    while !grid.all_flashed {
        grid.do_step();
        step_num += 1;
    }

    step_num
}

pub fn day11(input_lines: &[String]) -> (u64, u64) {
    let parsed_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&parsed_data) as u64, part_2(&parsed_data) as u64)
}
