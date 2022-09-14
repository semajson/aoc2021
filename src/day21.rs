use std::cmp;
use std::collections::HashMap;
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
        for dice_total_perm in get_dice_permutations(3, 3).iter() {
            let new_perm = permutations * dice_total_perm.permutations;

            if self.turn == 1 {
                let new_1_pos = get_new_position(self.player_1_position, dice_total_perm.total);
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
                let new_2_pos = get_new_position(self.player_2_position, dice_total_perm.total);
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
        println!("lowest_score is: {:?}", lowest_score);

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
                // unfinished, add it to the set
                let maybe_existing_count = unfinished_states.get_mut(&new_state);

                if let Some(existing_count) = maybe_existing_count {
                    *existing_count += permutations;
                } else {
                    // new state, add the state
                    unfinished_states.insert(new_state, permutations);
                }
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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DicePerm {
    total: isize,
    permutations: isize,
}
pub fn get_dice_permutations(max_value: isize, rolls: isize) -> Vec<DicePerm> {
    let mut dice_perumtations = vec![DicePerm {
        total: 0,
        permutations: 1,
    }];

    for _ in 0..rolls {
        let old_perm = dice_perumtations;
        dice_perumtations = vec![];

        for perm in old_perm.iter() {
            for value in 1..=max_value {
                let total = perm.total + value;

                let maybe_existing_value =
                    dice_perumtations.iter_mut().find(|x| (*x).total == total);

                if let Some(found_value) = maybe_existing_value {
                    found_value.permutations += perm.permutations;
                } else {
                    dice_perumtations.push(DicePerm {
                        total,
                        permutations: perm.permutations,
                    })
                }
            }
        }
    }
    dice_perumtations
}

#[test]
fn day21_test_get_dice_permutations() {
    assert!(
        get_dice_permutations(1, 1)
            == vec![DicePerm {
                total: 1,
                permutations: 1
            }]
    );

    assert!(get_dice_permutations(3, 1).len() == 3);

    assert!(get_dice_permutations(1, 3).len() == 1);

    assert!(get_dice_permutations(2, 2).len() == 3);

    assert!(get_dice_permutations(3, 2).len() == 5);
    assert!(
        get_dice_permutations(3, 2)
            == vec![
                DicePerm {
                    total: 2,
                    permutations: 1
                },
                DicePerm {
                    total: 3,
                    permutations: 2
                },
                DicePerm {
                    total: 4,
                    permutations: 3
                },
                DicePerm {
                    total: 5,
                    permutations: 2
                },
                DicePerm {
                    total: 6,
                    permutations: 1
                },
            ]
    );

    assert!(get_dice_permutations(2, 3).len() == 4);
    assert_eq!(
        get_dice_permutations(2, 3),
        vec![
            DicePerm {
                total: 3,
                permutations: 1
            },
            DicePerm {
                total: 4,
                permutations: 3
            },
            DicePerm {
                total: 5,
                permutations: 3
            },
            DicePerm {
                total: 6,
                permutations: 1
            },
        ]
    );

    assert!(get_dice_permutations(3, 3).len() == 7);
    assert_eq!(
        get_dice_permutations(3, 3)
            .iter()
            .map(|x| x.permutations)
            .sum::<isize>(),
        27
    );
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
