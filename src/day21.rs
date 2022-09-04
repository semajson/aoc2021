use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone)]
pub struct Player {
    position: usize,
    score: usize,
}
impl Player {
    pub fn new(input_line: &String) -> Player {
        let input_line = input_line.split(": ");
        let position = input_line.last().unwrap().parse::<usize>().unwrap();

        Player { position, score: 0 }
    }

    // pub fn count_lit_pixles(&self) -> usize {
    //     self.map
    //         .keys()
    //         .filter(|x| *self.map.get(*x).unwrap() == '#')
    //         .count()
    // }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<(Player, Player), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let player_1 = Player::new(input_lines[0]);
    let player_2 = Player::new(input_lines[1]);

    Ok((player_1, player_2))
}

pub fn part_1((player_1, player_2): (&Player, &Player)) -> i64 {
    let player_1 = player_1.clone();
    let player_2 = player_2.clone();

    // image.enhance(enhance_algo);
    // image.enhance(enhance_algo);

    // image.count_lit_pixles() as i64
    player_1.position as i64
}

pub fn part_2((player_1, player_2): (&Player, &Player)) -> i64 {
    let player_1 = player_1.clone();
    let player_2 = player_2.clone();

    0
}

pub fn day21(input_lines: &[String]) -> (u64, u64) {
    let (player_1, player_2) = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error {} when trying to parse the input lines", err);
    });

    (
        part_1((&player_1, &player_2)) as u64,
        part_2((&player_1, &player_2)) as u64,
    )
}
