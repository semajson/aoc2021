use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;
use std::num;

#[derive(Debug, Clone)]
pub struct Player {
    position: isize,
    score: isize,
}
impl Player {
    pub fn new(input_line: &str) -> Player {
        let input_line = input_line.split(": ");
        let position = input_line.last().unwrap().parse::<isize>().unwrap();

        Player { position, score: 0 }
    }

    pub fn do_turn(&mut self, rolls: &mut isize, dice_num: &mut isize) {
        let mut dice_total = 0;
        for _ in 0..3 {
            dice_total += *dice_num;
            *dice_num = (*dice_num % 100) + 1;
            *rolls += 1;
        }

        self.position = get_new_position(self.position, dice_total);

        self.score += self.position;
    }
}

fn get_new_position(curr_pos: isize, dice_total: isize) -> isize {
    let mut new_pos = curr_pos + dice_total;
    if new_pos > 10 {
        if (new_pos % 10) == 0 {
            new_pos = 10;
        } else {
            new_pos %= 10;
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

    let mut rolls = 0_isize;
    let mut dice_num = 1_isize;
    let mut turn = 1_isize;
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

        if player_1.score >= 1000 {
            return (rolls * player_2.score) as i64;
        } else if player_2.score >= 1000 {
            return (rolls * player_1.score) as i64;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    player_1_score: isize,
    player_1_position: isize,
    player_2_score: isize,
    player_2_position: isize,
    turn: isize,
}
impl State {
    pub fn get_next_states(self, permutations: isize) -> Vec<(State, isize)> {
        let mut new_states = vec![];
        for (dice_total, dice_perm) in get_dice_permutations(3, 3).iter() {
            let new_perm = permutations * dice_perm;

            if self.turn == 1 {
                let new_1_pos = get_new_position(self.player_1_position, *dice_total);
                new_states.push((
                    State {
                        player_1_score: self.player_1_score + new_1_pos,
                        player_1_position: new_1_pos,
                        player_2_score: self.player_2_score,
                        player_2_position: self.player_2_position,
                        turn: 2,
                    },
                    new_perm,
                ));
            } else if self.turn == 2 {
                let new_2_pos = get_new_position(self.player_2_position, *dice_total);
                new_states.push((
                    State {
                        player_1_score: self.player_1_score,
                        player_1_position: self.player_1_position,
                        player_2_score: self.player_2_score + new_2_pos,
                        player_2_position: new_2_pos,
                        turn: 1,
                    },
                    new_perm,
                ));
            }
        }

        new_states
    }
}

pub fn part_2((player_1, player_2): (&Player, &Player)) -> i64 {
    let player_1 = player_1.clone();
    let player_2 = player_2.clone();

    let init_state = State {
        player_1_score: player_1.score,
        player_1_position: player_1.position,
        player_2_score: player_2.score,
        player_2_position: player_2.position,
        turn: 1,
    };

    let mut wins_1 = 0;
    let mut wins_2 = 0;

    let mut unfinished_states: HashMap<State, isize> = HashMap::new();
    unfinished_states.insert(init_state, 1);

    while !unfinished_states.is_empty() {
        // get the lowest scoring state
        let mut lowest_score = 9999;
        let mut lowest_state = None;
        let mut lowest_permutations = None;

        // should this be lowest state whos player turn it is?
        for (state, permutations) in unfinished_states.iter() {
            if state.player_1_score < lowest_score {
                lowest_score = state.player_1_score;
                lowest_state = Some(state);
                lowest_permutations = Some(permutations);
            }
            if state.player_2_score < lowest_score {
                lowest_score = state.player_2_score;
                lowest_state = Some(state);
                lowest_permutations = Some(permutations);
            }
        }
        // println!("lowest_score is: {:?}", lowest_score);

        let lowest_state = lowest_state.unwrap().clone();
        let lowest_permutations = *lowest_permutations.unwrap();

        unfinished_states.remove(&lowest_state);

        // replace this with new states
        for (new_state, permutations) in lowest_state
            .get_next_states(lowest_permutations)
            .into_iter()
        {
            if new_state.player_1_score >= 21 {
                wins_1 += permutations;
            } else if new_state.player_2_score >= 21 {
                wins_2 += permutations;
            } else {
                let new_permutation = unfinished_states.entry(new_state).or_insert(0);
                *new_permutation += permutations;
            }
        }
    }

    (cmp::max(wins_1, wins_2)) as i64
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

pub fn get_dice_permutations(max_value: isize, rolls: isize) -> HashMap<isize, isize> {
    let mut dice_perumtations = HashMap::new();
    dice_perumtations.insert(0, 1);

    for _ in 0..rolls {
        let mut new_dice_perumtations = HashMap::new();

        for (total, permutations) in dice_perumtations.iter() {
            for roll_value in 1..=max_value {
                let new_total = total + roll_value;

                let new_permutation = new_dice_perumtations.entry(new_total).or_insert(0);
                *new_permutation += permutations;
            }
        }
        dice_perumtations = new_dice_perumtations;
    }

    dice_perumtations
}

#[allow(dead_code)]
pub fn get_dice_perm_recr_map(max_value: isize, rolls: isize) -> HashMap<isize, isize> {
    let mut perm_map = HashMap::new();
    get_dice_perm_recr(max_value, rolls, &mut perm_map, 0);
    perm_map
}

#[allow(dead_code)]
pub fn get_dice_perm_recr(
    max_value: isize,
    rolls: isize,
    perm_map: &mut HashMap<isize, isize>,
    total: isize,
) {
    if rolls == 0 {
        let perumtations = perm_map.entry(total).or_insert(0);
        *perumtations += 1;
    } else {
        for roll_value in 1..=max_value {
            get_dice_perm_recr(max_value, rolls - 1, perm_map, total + roll_value);
        }
    }
}

#[test]
fn day21_test_get_dice_permutations() {
    assert_eq!(get_dice_permutations(1, 1)[&1], 1);

    assert!(get_dice_permutations(3, 1).len() == 3);

    assert!(get_dice_permutations(1, 3).len() == 1);

    assert!(get_dice_permutations(2, 2).len() == 3);

    assert!(get_dice_permutations(3, 2).len() == 5);

    assert_eq!(get_dice_permutations(3, 2)[&2], 1);
    assert_eq!(get_dice_permutations(3, 2)[&3], 2);
    assert_eq!(get_dice_permutations(3, 2)[&4], 3);
    assert_eq!(get_dice_permutations(3, 2)[&5], 2);
    assert_eq!(get_dice_permutations(3, 2)[&6], 1);

    assert!(get_dice_permutations(2, 3).len() == 4);

    assert!(get_dice_permutations(3, 3).len() == 7);
}

#[test]
fn day21_test_get_dice_perm_recr_map() {
    assert_eq!(get_dice_perm_recr_map(1, 1)[&1], 1);

    assert!(get_dice_perm_recr_map(3, 1).len() == 3);

    assert!(get_dice_perm_recr_map(1, 3).len() == 1);

    assert!(get_dice_perm_recr_map(2, 2).len() == 3);

    assert!(get_dice_perm_recr_map(3, 2).len() == 5);

    assert_eq!(get_dice_perm_recr_map(3, 2)[&2], 1);
    assert_eq!(get_dice_perm_recr_map(3, 2)[&3], 2);
    assert_eq!(get_dice_perm_recr_map(3, 2)[&4], 3);
    assert_eq!(get_dice_perm_recr_map(3, 2)[&5], 2);
    assert_eq!(get_dice_perm_recr_map(3, 2)[&6], 1);

    assert!(get_dice_perm_recr_map(2, 3).len() == 4);

    assert!(get_dice_perm_recr_map(3, 3).len() == 7);
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
