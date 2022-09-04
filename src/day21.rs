use std::collections::HashMap;
use std::collections::HashSet;
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

        self.position = get_new_position(self.position, dice_total);
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

fn get_new_position(curr_pos: isize, dice_total: isize) -> isize {
    let mut new_pos = curr_pos + dice_total;
    if new_pos > 10 {
        if (new_pos % 10) == 0 {
            new_pos = 10;
        } else {
            new_pos = (new_pos % 10);
        }
    }
    new_pos
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

#[derive(PartialEq, Eq, Hash)]
struct State {
    player_1_score: isize,
    player_1_position: isize,
    player_2_score: isize,
    player_2_position: isize,
    turn: isize,
}
impl State {
    pub fn get_next_states(self) -> Vec<(State, isize)> {
        let mut new_states = vec![];
        for roll in 1..=3 {
            match self.turn {
                1 => {}
                2 => {}
                _ => panic!("invalid branch"),
            }
        }

        new_states
    }
}

pub fn part_2((player_1, player_2): (&Player, &Player)) -> i64 {
    let mut player_1 = player_1.clone();
    let mut player_2 = player_2.clone();

    let init_state = State {
        player_1_score: player_1.score,
        player_1_position: player_1.position,
        player_2_score: player_2.score,
        player_2_position: player_2.position,
        turn: 1,
    };

    let wins_1 = 0;
    let wins_2 = 0;

    let mut unfinished_states: HashMap<State, isize> = HashMap::new();
    unfinished_states.insert(init_state, 1);

    while !unfinished_states.is_empty() {
        // get the lowest scoring state
        let (state, _) = unfinished_states.into_iter().next().unwrap();

        // replace this with new states
        for (new_state, permutations) in state.get_next_states().into_iter() {
            // if winner

            // ah, this fundementally won't work, as can't get a mutable reference to a hashstate!
            // lets do with with a hashmap instead....
            let maybe_existing_count = unfinished_states.get_mut(&new_state);

            if let Some(existing_count) = maybe_existing_count {
                *existing_count += permutations;
            } else {
                unfinished_states.insert(new_state, permutations);
            }
        }
    }

    ((wins_1 - wins_2) as i64).abs() as i64
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
