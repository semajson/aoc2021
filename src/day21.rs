use std::collections::HashMap;
use std::num;

#[derive(Debug, Clone)]
pub struct Player {
    position: isize,
    score: isize,
}
impl Player {
    pub fn new(input_line: &String) -> Player {
        let input_line = input_line.split(": ");
        let position = input_line.last().unwrap().parse::<isize>().unwrap();

        Player { position, score: 0 }
    }

    pub fn do_turn(&mut self, rolls: &mut isize, dice_num: &mut isize) {
        if *dice_num > 97 {
            println!("in here");
        }
        let mut dice_total = 0;
        for _ in 0..3 {
            dice_total += *dice_num;
            *dice_num = (*dice_num % 100) + 1;
            *rolls += 1;
        }

        self.position = self.position + dice_total;
        if self.position > 10 {
            if (self.position % 10) == 0 {
                self.position = 10;
            } else {
                self.position = (self.position % 10);
            }
        }
        self.score += self.position;
    }
    pub fn do_dirac_turn(&mut self, rolls: &mut isize, dice_num: &mut isize) {
        if *dice_num > 97 {
            println!("in here");
        }
        let mut dice_total = 0;
        for _ in 0..3 {
            dice_total += *dice_num;
            *dice_num = (*dice_num % 100) + 1;
            *rolls += 1;
        }

        self.position = self.position + dice_total;
        if self.position > 10 {
            if (self.position % 10) == 0 {
                self.position = 10;
            } else {
                self.position = (self.position % 10);
            }
        }
        self.score += self.position;
    }
}

fn parse_input_lines(raw_input_lines: &[String]) -> Result<(Player, Player), num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();

    let player_1 = Player::new(input_lines[0]);
    let player_2 = Player::new(input_lines[1]);

    Ok((player_1, player_2))
}

pub fn part_1((player_1, player_2): (&Player, &Player)) -> i64 {
    let mut player_1 = player_1.clone();
    let mut player_2 = player_2.clone();

    // image.enhance(enhance_algo);
    // image.enhance(enhance_algo);
    let mut rolls = 0 as isize;
    let mut dice_num = 1 as isize;
    let mut turn = 1 as isize;
    loop {
        match turn {
            1 => {
                player_1.do_turn(&mut rolls, &mut dice_num);
                turn = 2;
            }
            2 => {
                player_2.do_turn(&mut rolls, &mut dice_num);
                turn = 1;
            }
            _ => panic!("invalid branch"),
        }

        if (player_1.score >= 1000) {
            println!("rolls is:{:}", rolls);
            println!("player2_score is:{:}", player_2.score);
            return (rolls * player_2.score) as i64;
        } else if (player_2.score >= 1000) {
            println!("rolls is:{:}", rolls);
            println!("player1_score is:{:}", player_1.score);
            return (rolls * player_2.score) as i64;
        }
    }

    panic!("error");
}

pub fn part_2((player_1, player_2): (&Player, &Player)) -> i64 {
    let mut player_1 = player_1.clone();
    let mut player_2 = player_2.clone();

    // image.enhance(enhance_algo);
    // image.enhance(enhance_algo);
    let mut rolls = 0 as isize;
    let mut dice_num = 1 as isize;
    let mut turn = 1 as isize;
    loop {
        match turn {
            1 => {
                player_1.do_dirac_turn(&mut rolls, &mut dice_num);
                turn = 2;
            }
            2 => {
                player_2.do_dirac_turn(&mut rolls, &mut dice_num);
                turn = 1;
            }
            _ => panic!("invalid branch"),
        }

        if (player_1.score >= 1000) {
            println!("rolls is:{:}", rolls);
            println!("player2_score is:{:}", player_2.score);
            return (rolls * player_2.score) as i64;
        } else if (player_2.score >= 1000) {
            println!("rolls is:{:}", rolls);
            println!("player1_score is:{:}", player_1.score);
            return (rolls * player_2.score) as i64;
        }
    }
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

#[test]
fn day21_part1_tests() {
    let mut player_1 = Player {
        position: 4,
        score: 0,
    };
    let mut player_2 = Player {
        position: 8,
        score: 0,
    };
    let mut rolls = 1;
    let mut dice_num = 1;
    player_1.do_turn(&mut rolls, &mut dice_num);
    assert!(player_1.score == 10);

    player_2.do_turn(&mut rolls, &mut dice_num);
    assert!(player_2.score == 3);

    player_1.do_turn(&mut rolls, &mut dice_num);
    assert!(player_1.score == 14);

    player_2.do_turn(&mut rolls, &mut dice_num);
    assert!(player_2.score == 9);

    player_1.do_turn(&mut rolls, &mut dice_num);
    assert!(player_1.score == 20);

    player_2.do_turn(&mut rolls, &mut dice_num);
    assert!(player_2.score == 16);

    let mut player_1 = Player {
        position: 4,
        score: 990,
    };
    let mut player_2 = Player {
        position: 6,
        score: 742,
    };
    let mut rolls = 1;
    let mut dice_num = 88;

    player_2.do_turn(&mut rolls, &mut dice_num);
    assert!(player_2.score == 745);

    player_1.do_turn(&mut rolls, &mut dice_num);
    assert!(player_1.score == 1000);
}
