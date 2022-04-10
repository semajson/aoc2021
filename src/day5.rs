use std::cmp;
use std::cmp::Ordering;
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
}

impl Vent {
    pub fn new(line: &str) -> Result<Vent, num::ParseIntError> {
        let mut vent_ends = Vec::new();
        let cords = line.split(" -> ").collect::<Vec<&str>>();

        for cord in cords.iter() {
            let cord = cord
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse::<i32>().unwrap()) // james, how raise error during map?
                .collect::<Vec<i32>>();

            vent_ends.push(Cord {
                x: cord[0],
                y: cord[1],
            });
        }

        // work out if horz or vert
        let mut all_cords = Vec::new();
        let mut horz_or_vert = false;

        if (vent_ends[0].x == vent_ends[1].x) || (vent_ends[0].y == vent_ends[1].y) {
            horz_or_vert = true;
        }

        // coords for this vent
        if vent_ends[0].x == vent_ends[1].x {
            // deal with horz lines differently
            let y_min = cmp::min(vent_ends[0].y, vent_ends[1].y);
            let y_max = cmp::max(vent_ends[0].y, vent_ends[1].y);
            for y in y_min..=y_max {
                all_cords.push(Cord {
                    x: vent_ends[0].x,
                    y,
                })
            }
        } else {
            let step;
            match vent_ends[1].y.cmp(&vent_ends[0].y) {
                Ordering::Greater => step = 1,
                Ordering::Less => step = -1,
                Ordering::Equal => step = 0,
            }

            let mut y_offset = 0;

            if vent_ends[1].x > vent_ends[0].x {
                for x_cord in vent_ends[0].x..=vent_ends[1].x {
                    all_cords.push(Cord {
                        x: x_cord,
                        y: vent_ends[0].y + y_offset,
                    });
                    y_offset += step;
                }
            } else {
                for x_cord in (vent_ends[1].x..=vent_ends[0].x).rev() {
                    all_cords.push(Cord {
                        x: x_cord,
                        y: vent_ends[0].y + y_offset,
                    });
                    y_offset += step;
                }
            }
        }

        Ok(Vent {
            cords: all_cords,
            horz_or_vert,
        })
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
    // let mut boards = (*raw_boards).clone();
    let mut board = Diagram::new(1000);
    let mut num_more_1 = 0;
    for vent in vents {
        if vent.horz_or_vert {
            for cord in &vent.cords {
                if board.0[cord.x as usize][cord.y as usize] == 1 {
                    num_more_1 += 1;
                }
                board.0[cord.x as usize][cord.y as usize] += 1;
            }
        }
    }

    num_more_1
}

pub fn part_2(vents: &Vec<Vent>) -> i32 {
    // let mut boards = (*raw_boards).clone();
    let mut board = Diagram::new(1000);
    let mut num_more_1 = 0;
    for vent in vents {
        for cord in &vent.cords {
            if board.0[cord.x as usize][cord.y as usize] == 1 {
                num_more_1 += 1;
            }
            board.0[cord.x as usize][cord.y as usize] += 1;
        }
    }

    num_more_1
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<Vec<Vent>, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
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
