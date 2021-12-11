use std::num;

#[derive(Debug, Clone)]
pub struct DrawnNumbers(Vec<i32>);
impl DrawnNumbers {
    pub fn new(line: &str) -> Result<DrawnNumbers, num::ParseIntError> {
        let drawn_numbers = line
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // how raise error during map?
        Ok(DrawnNumbers(drawn_numbers))
    }
    pub fn iter(&self) -> std::slice::Iter<'_, i32> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    value: i32,
    marked: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board(Vec<Vec<Cell>>);
impl Board {
    pub fn new(lines: Vec<&str>) -> Result<Board, num::ParseIntError> {
        let mut board = Vec::new();

        for line in lines {
            let row = line.to_string();
            let row = row.replace("  ", " ");
            let row = row.split(' ').collect::<Vec<&str>>();

            let parsed_row = row
                .iter()
                .filter(|x| !x.is_empty())
                .map(|num| Cell {
                    value: num.parse::<i32>().unwrap(),
                    marked: false,
                })
                .collect::<Vec<Cell>>();
            board.push(parsed_row);
            // how raise error during map?
        }
        Ok(Board(board))
    }
    pub fn mark_num(&mut self, num_drawn: i32) {
        // this isn't great, should have hash map on board instead
        // to mark nums as that would be more performant, but hey ho
        for row in self.0.iter_mut() {
            for mut cell in row {
                if cell.value == num_drawn {
                    cell.marked = true;
                    return;
                }
            }
        }
    }
    pub fn calc_is_bingo(&self) -> bool {
        // check rows
        for row in &self.0 {
            let mut have_bingo = true;
            for cell in row {
                if !cell.marked {
                    have_bingo = false;
                    break;
                }
            }
            if have_bingo {
                return true;
            }
        }

        // check columns
        for i in 0..self.0.len() {
            let mut have_bingo = true;
            for row in &self.0 {
                if !row[i].marked {
                    have_bingo = false;
                    break;
                }
            }
            if have_bingo {
                return true;
            }
        }

        // not bingo
        false
    }

    pub fn get_score(&self) -> i32 {
        let mut score = 0;

        for row in &self.0 {
            for cell in row {
                if !cell.marked {
                    score += cell.value;
                }
            }
        }
        score
    }
}

pub fn part_1(drawn_numbers: &DrawnNumbers, raw_boards: &Vec<Board>) -> i32 {
    println!("hello world");
    let mut boards = (*raw_boards).clone();

    for num in drawn_numbers.iter() {
        for board in boards.iter_mut() {
            // let board = &mut boards[i];
            board.mark_num(*num);
            if board.calc_is_bingo() {
                return board.get_score() * num;
            }
        }
    }
    0
}

pub fn part_2(drawn_numbers: &DrawnNumbers, raw_boards: &Vec<Board>) -> i32 {
    println!("hello world");
    let mut boards = (*raw_boards).clone();

    let mut remaining_boards = Vec::new();

    for index in 0..boards.len() {
        remaining_boards.push(index);
    }

    for num in drawn_numbers.iter() {
        for i in 0..boards.len() {
            let board = &mut boards[i];

            // will be a more efficient way to do this when board was previously bingo...
            if board.calc_is_bingo() {
                continue;
            }

            board.mark_num(*num);

            if board.calc_is_bingo() {
                if remaining_boards.len() == 1 {
                    return boards[remaining_boards[0]].get_score() * num;
                }
                remaining_boards = remaining_boards
                    .into_iter()
                    .filter(|x| *x != i)
                    .collect::<Vec<usize>>();
            }
        }
    }
    0
}

fn parse_input_lines(
    raw_input_lines: &[String],
) -> Result<(DrawnNumbers, Vec<Board>), num::ParseIntError> {
    let mut input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    // First get the drawn numbers
    let drawn_numbers = DrawnNumbers::new(input_lines.remove(0))?;

    // Build all the boards
    let mut boards = Vec::new();
    while !input_lines.is_empty() {
        // Now get past the gap
        if input_lines[0].is_empty() {
            input_lines.remove(0);
        }

        // get the current
        let mut current_board = Vec::new();
        while (!input_lines.is_empty()) && (!input_lines[0].is_empty()) {
            current_board.push(input_lines.remove(0).as_str());
        }
        boards.push(Board::new(current_board)?);
    }

    Ok((drawn_numbers, boards))
}

pub fn day4(input_lines: &[String]) -> (u64, u64) {
    let (drawn_numbers, boards) = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1(&drawn_numbers, &boards) as u64,
        part_2(&drawn_numbers, &boards) as u64,
    )
}
