#[derive(Debug)]
pub struct Position {
    x: isize,
    z: isize,
    aim: isize,
}

#[derive(Debug)]
pub struct Command {
    direction: String,
    amount: isize,
}

impl Command {
    pub fn new(line: &str) -> Result<Command, &'static str> {
        let line = line.split(' ').collect::<Vec<&str>>();

        if line.len() != 2 {
            return Err("Incorrect number or arguments in command");
        }

        let direction = line[0].to_owned();
        let amount = line[1].parse::<isize>().unwrap();

        Ok(Command { direction, amount })
    }
    pub fn run(&self, mut curr_pos: Position) -> Position {
        match self.direction.as_str() {
            "forward" => curr_pos.x += self.amount,
            "up" => curr_pos.z -= self.amount,
            "down" => curr_pos.z += self.amount,
            _ => panic!("Unknown command!"),
        }
        curr_pos
    }
    pub fn run_part2(&self, mut curr_pos: Position) -> Position {
        match self.direction.as_str() {
            "forward" => {
                curr_pos.x += self.amount;
                curr_pos.z += curr_pos.aim * self.amount
            }
            "up" => curr_pos.aim -= self.amount,
            "down" => curr_pos.aim += self.amount,
            _ => panic!("Unknown command!"),
        }
        curr_pos
    }
}

fn parse_input_lines(input_lines: &[String]) -> Result<Vec<Command>, &'static str> {
    let mut parsed_data = Vec::new();

    for line in input_lines {
        parsed_data.push(Command::new(line)?);
    }
    Ok(parsed_data)
}

pub fn part_1(parsed_data: &[Command]) -> isize {
    let mut curr_pos = Position { x: 0, z: 0, aim: 0 };

    for command in parsed_data {
        curr_pos = command.run(curr_pos);
    }

    curr_pos.x * curr_pos.z
}

pub fn part_2(parsed_data: &[Command]) -> isize {
    let mut curr_pos = Position { x: 0, z: 0, aim: 0 };

    for command in parsed_data {
        curr_pos = command.run_part2(curr_pos);
    }

    curr_pos.x * curr_pos.z
}

pub fn day2(input_lines: &[String]) -> (u64, u64) {
    let parsed_test_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1(&parsed_test_data) as u64,
        part_2(&parsed_test_data) as u64,
    )
}
