use std::collections::HashSet;
use std::fmt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::num;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct PaperGrid {
    dots: HashSet<Dot>,
}
impl PaperGrid {
    pub fn do_fold(&mut self, fold: &Fold) {
        let mut remove_dots = vec![];
        let mut add_dots = vec![];

        for dot in self.dots.iter() {
            if let Some(fold_x) = fold.x {
                if ((dot.x as isize) - (fold_x as isize)) > 0 {
                    let new_dot = Dot {
                        x: fold_x - (dot.x - fold_x),
                        y: dot.y,
                    };
                    remove_dots.push(dot.clone());
                    add_dots.push(new_dot)
                }
            } else if let Some(fold_y) = fold.y {
                if ((dot.y as isize) - (fold_y as isize)) > 0 {
                    let new_dot = Dot {
                        x: dot.x,
                        y: fold_y - (dot.y - fold_y),
                    };
                    remove_dots.push(dot.clone());
                    add_dots.push(new_dot)
                }
            }
        }

        for dot in remove_dots.iter() {
            self.dots.remove(dot);
        }

        for dot in add_dots.into_iter() {
            self.dots.insert(dot);
        }
    }

    pub fn get_debug_view(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("outputs/day13")
            .unwrap();

        let max_x = self.dots.iter().max_by_key(|p| p.x).unwrap().x + 1;
        let max_y = self.dots.iter().max_by_key(|p| p.y).unwrap().y + 1;

        let mut grid = vec![vec!["."; (max_x) as usize]; (max_y) as usize];

        for dot in self.dots.iter() {
            grid[dot.y][dot.x] = "#";
        }
        for line in grid.iter() {
            let line_formatted = line.join("");
            println!("{:?}", line_formatted);
            if let Err(e) = writeln!(file, "{:?}", line_formatted) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
}
impl fmt::Display for PaperGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_x = 11;
        let max_y = 15;

        let mut grid = vec![vec!["."; (max_y) as usize]; (max_x) as usize];

        for dot in self.dots.iter() {
            grid[dot.x][dot.y] = "#";
        }

        write!(f, "{:?}", grid)
    }
}

pub struct Fold {
    x: Option<usize>,
    y: Option<usize>,
}

fn parse_input_lines(
    raw_input_lines: &[String],
) -> Result<(PaperGrid, Vec<Fold>), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    let mut input_lines = input_lines.clone();

    // Get dots
    let mut dots = HashSet::new();
    let mut line = input_lines.remove(0);
    while !line.is_empty() {
        let dot = line.split(',').collect::<Vec<&str>>();

        dots.insert(Dot {
            x: dot[0].parse::<usize>()?,
            y: dot[1].parse::<usize>()?,
        });

        line = input_lines.remove(0);
    }

    // Get Folds
    let mut folds = vec![];
    loop {
        let line = input_lines.remove(0);
        let fold = line.replace("fold along ", "");
        let fold = fold.split('=').collect::<Vec<&str>>();

        match fold[0] {
            "x" => folds.push(Fold {
                x: Some(fold[1].parse::<usize>()?),
                y: None,
            }),
            "y" => folds.push(Fold {
                x: None,
                y: Some(fold[1].parse::<usize>()?),
            }),
            _ => panic!("unreachable"),
        }

        if input_lines.is_empty() {
            break;
        }
    }

    Ok((PaperGrid { dots }, folds))
}

pub fn part_1(paper_grid: &PaperGrid, folds: &Vec<Fold>) -> i64 {
    let mut paper_grid = paper_grid.clone();

    for fold in folds.iter() {
        paper_grid.do_fold(fold);
    }

    paper_grid.dots.len() as i64
}

pub fn part_2(paper_grid: &PaperGrid, folds: &Vec<Fold>) -> i64 {
    let mut paper_grid = paper_grid.clone();

    for fold in folds.iter() {
        paper_grid.do_fold(fold);
    }
    paper_grid.get_debug_view();

    paper_grid.dots.len() as i64
}

pub fn day13(input_lines: &[String]) -> (u64, u64) {
    let (paper_grid, folds) = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });

    (
        part_1(&paper_grid, &folds) as u64,
        part_2(&paper_grid, &folds) as u64,
    )
}
