use std::cmp;
use std::num;

#[derive(Debug, Clone)]
pub struct Cord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Vent {
    cords: Vec<Cord>,
    horz_or_vert: bool,
    // vert: bool,
}

impl Vent {
    pub fn new(line: &str) -> Result<Vent, num::ParseIntError> {
        let mut cord_ends = Vec::new();
        let cords = line.split(" -> ").collect::<Vec<&str>>();

        for cord in cords.iter() {
            let cord = cord
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse::<i32>().unwrap()) // james, how raise error during map?
                .collect::<Vec<i32>>();

            cord_ends.push(Cord {
                x: cord[0],
                y: cord[1],
            });
        }

        // work out if horz or vert
        let mut all_cords = Vec::new();
        let mut horz_or_vert = false;
        if (cord_ends[0].x == cord_ends[1].x) {
            // vert
            let y_min = cmp::min(cord_ends[0].y, cord_ends[1].y);
            let y_max = cmp::max(cord_ends[0].y, cord_ends[1].y);
            for y in y_min..(y_max + 1) {
                all_cords.push(Cord {
                    x: cord_ends[0].x,
                    y: y,
                })
            }
            horz_or_vert = true;
        } else if (cord_ends[0].y == cord_ends[1].y) {
            // horz
            let x_min = cmp::min(cord_ends[0].x, cord_ends[1].x);
            let x_max = cmp::max(cord_ends[0].x, cord_ends[1].x);

            for x in x_min..(x_max + 1) {
                all_cords.push(Cord {
                    x: x,
                    y: cord_ends[0].y,
                })
            }
            horz_or_vert = true;
        } else {
            // diag
            let mut x_min = None;
            let mut x_max = None;

            let mut y_when_x_min = None;
            let mut y_when_x_max = None;

            if cord_ends[0].x > cord_ends[1].x {
                x_min = Some(cord_ends[1].x);
                y_when_x_min = Some(cord_ends[1].y);

                x_max = Some(cord_ends[0].x);
                y_when_x_max = Some(cord_ends[0].y);
            } else {
                x_min = Some(cord_ends[0].x);
                y_when_x_min = Some(cord_ends[0].y);

                x_max = Some(cord_ends[1].x);
                y_when_x_max = Some(cord_ends[1].y);
            }

            // let y_min = cmp::min(cord_ends[0].y, cord_ends[1].y);
            // let y_max = cmp::max(cord_ends[0].y, cord_ends[1].y);
            if y_when_x_min < y_when_x_max {
                for i in 0..(x_max.unwrap() + 1 - x_min.unwrap()) {
                    all_cords.push(Cord {
                        x: x_min.unwrap() + i,
                        y: y_when_x_min.unwrap() + i,
                    })
                }
            } else {
                for i in 0..(x_max.unwrap() + 1 - x_min.unwrap()) {
                    all_cords.push(Cord {
                        x: x_min.unwrap() + i,
                        y: y_when_x_min.unwrap() - i,
                    })
                }
            }
            horz_or_vert = false;
        }

        Ok(Vent {
            cords: all_cords,
            horz_or_vert: horz_or_vert,
        })
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Cord> {
        self.cords.iter()
    }
    pub fn into_iter(self) -> std::vec::IntoIter<Cord> {
        self.cords.into_iter()
    }
}

pub struct Diagram(Vec<Vec<i32>>);
impl Diagram {
    fn new(size: usize) -> Diagram {
        let mut diagram = Vec::new();
        for _ in 0..size {
            diagram.push(vec![0; size]);
        }
        Diagram(diagram)
    }
}

pub fn part_1(vents: &Vec<Vent>) -> i32 {
    println!("hello world");
    // let mut boards = (*raw_boards).clone();
    let mut board = Diagram::new(1000);
    let mut num_more_1 = 0;
    for vent in vents {
        if vent.horz_or_vert {
            for cord in &vent.cords {
                if (board.0[cord.x as usize][cord.y as usize] == 1) {
                    num_more_1 += 1;
                }
                board.0[cord.x as usize][cord.y as usize] += 1;
            }
        }
    }

    num_more_1
}

pub fn part_2(vents: &Vec<Vent>) -> i32 {
    println!("hello world");
    // let mut boards = (*raw_boards).clone();
    let mut board = Diagram::new(1000);
    let mut num_more_1 = 0;
    for vent in vents {
        for cord in &vent.cords {
            if (board.0[cord.x as usize][cord.y as usize] == 1) {
                num_more_1 += 1;
            }
            board.0[cord.x as usize][cord.y as usize] += 1;
        }
    }

    num_more_1
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Vec<Vent>, num::ParseIntError> {
    let mut input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    // First get the drawn numbers
    let mut vents = Vec::new();
    for line in input_lines {
        vents.push(Vent::new(line)?);
    }

    Ok(vents)
}

pub fn day5(input_lines: &[String]) -> (u64, u64) {
    let vents = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (part_1(&vents) as u64, part_2(&vents) as u64)
}
