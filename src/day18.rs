// use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt;
use std::num;

#[derive(Clone)]
enum SnailfishNumberOption {
    Raw(i32),
    Pair(Vec<Box<SnailfishNumber>>),
}

#[derive(Clone, Copy)]
pub struct ExplodeResult {
    exploded: bool,
    left_carry: Option<i32>,
    right_carry: Option<i32>,
}

#[derive(Clone)]
pub struct SnailfishNumber {
    number: SnailfishNumberOption,
}
impl SnailfishNumber {
    pub fn new(line: &str) -> Result<SnailfishNumber, num::ParseIntError> {
        // [[[[4,3],4],4],[7,[[8,4],9]]]

        // ok check ends and starts with []
        assert!(line.chars().nth(0).unwrap() == '[' && line.chars().last().unwrap() == ']');

        // remove them
        let line_len = line.len();
        let line = &line[1..line_len - 1];

        // then, look for the middle ,
        // walk through the string, have a record stack of [] seen,
        // stack is empty and seen a , - then we know it is the middle, so split again
        let mut middle_comma_index = None;
        let mut current_bracket_stack = vec![];

        for (index, x) in line.chars().enumerate() {
            match x {
                '[' => {
                    current_bracket_stack.push('[');
                }
                ']' => {
                    current_bracket_stack.pop();
                }
                ',' => {
                    if current_bracket_stack.is_empty() {
                        middle_comma_index = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let left_num_str = &line[..middle_comma_index.unwrap()];
        let left_num;
        if left_num_str.starts_with('[') {
            left_num = Box::new(SnailfishNumber::new(left_num_str).unwrap());
        } else {
            left_num = Box::new(SnailfishNumber {
                number: SnailfishNumberOption::Raw(left_num_str.parse::<i32>().unwrap()),
            });
        }

        let right_num_str = &line[middle_comma_index.unwrap() + 1..line.len()];
        let right_num;
        if right_num_str.starts_with('[') {
            right_num = Box::new(SnailfishNumber::new(right_num_str).unwrap());
        } else {
            right_num = Box::new(SnailfishNumber {
                number: SnailfishNumberOption::Raw(right_num_str.parse::<i32>().unwrap()),
            });
        }

        let number_pair = vec![left_num, right_num];

        Ok(SnailfishNumber {
            number: SnailfishNumberOption::Pair(number_pair),
        })
    }
    // pub fn add(mut self, other_number: &SnailfishNumber) -> SnailfishNumber {
    //     // [1,2] + [[3,4],5] = [[1,2],[[3,4],5]]
    //     self = SnailfishNumber {
    //         number: SnailfishNumberOption::Pair(vec![
    //             Box::new(self),
    //             Box::new(other_number.clone()),
    //         ]),
    //         parent: None,
    //     };
    //     self.reduce();
    //     self
    // }
    pub fn reduce(&mut self) -> () {
        ()
        // Try explode, then try split, then repeat.
        // If no explode or split, then end
        // loop {
        //     if self.maybe_explode() {
        //         println!("Exploded!");
        //     } else if self.maybe_split() {
        //         println!("Split");
        //     } else {
        //         println!("Finished!");
        //         break;
        //     }
        // }
    }
    pub fn maybe_split(&mut self) -> bool {
        match &mut self.number {
            SnailfishNumberOption::Raw(i) => {
                if *i > 9 {
                    let new_left_num = Box::new(SnailfishNumber {
                        number: SnailfishNumberOption::Raw(((*i as f32) / 2_f32).floor() as i32),
                    });
                    let new_right_num = Box::new(SnailfishNumber {
                        number: SnailfishNumberOption::Raw(((*i as f32) / 2_f32).ceil() as i32),
                    });
                    let new_pair = vec![new_left_num, new_right_num];

                    self.number = SnailfishNumberOption::Pair(new_pair);
                    return true;
                } else {
                    return false;
                }
            }
            SnailfishNumberOption::Pair(i) => {
                if i[0].maybe_split() {
                    println!("let split");
                    return true;
                } else if i[1].maybe_split() {
                    println!("right split");
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
    pub fn maybe_explode(&mut self) -> bool {
        // Find first 4 deep pair
        // let pair_to_explode = self.find_pair_to_explode(0);

        // if let Some(pair) = pair_to_explode {
        //     println!("{:?}", pair);
        // }
        let mut explode_result = ExplodeResult {
            exploded: false,
            left_carry: None,
            right_carry: None,
        };
        let explode_result = self.maybe_do_explode(0, explode_result);

        if explode_result.exploded {
            return true;
        }
        false
    }

    pub fn maybe_do_explode(
        &mut self,
        depth: u32,
        mut explode_result: ExplodeResult,
    ) -> ExplodeResult {
        // What is want:
        // recursive
        // if 4 deep:
        //      explode, create explode object and return it
        // then if have explode object, need to remove the num
        // and deal with adding the number to left and right

        if let SnailfishNumberOption::Pair(pair) = &mut self.number {
            // deal with result first
            if depth == 4 {
                explode_result.exploded = true;
                if let SnailfishNumberOption::Raw(left_num) = pair[0].number {
                    explode_result.left_carry = Some(left_num);
                } else {
                    panic!("unexpected - left num is pair");
                }
                if let SnailfishNumberOption::Raw(right_num) = pair[1].number {
                    explode_result.right_carry = Some(right_num);
                } else {
                    panic!("unexpected - right num is pair");
                }
                self.number = SnailfishNumberOption::Raw(0);
            } else {
                explode_result = pair[0].maybe_do_explode(depth + 1, explode_result);
                if explode_result.exploded {
                    // carry right
                    self.carry_right(&mut explode_result);
                } else {
                    explode_result = pair[1].maybe_do_explode(depth + 1, explode_result);
                    if explode_result.exploded {
                        // carry left
                        self.carry_left(&mut explode_result);
                    }
                }
            }
        }

        return explode_result;
    }

    pub fn carry_right(&mut self, explode_result: &mut ExplodeResult) {
        if let SnailfishNumberOption::Pair(pair) = &mut self.number {
            if let Some(right_carry) = explode_result.right_carry {
                match &pair[1].number {
                    SnailfishNumberOption::Raw(mut right_num) => {
                        println!("Right carry is is{:?}", explode_result.right_carry);
                        println!("current num is: is is{:?}", right_num);

                        pair[1].number = SnailfishNumberOption::Raw(right_num + right_carry);
                        // println!("after right is{:?}", self);
                        explode_result.right_carry = None;
                    }
                    SnailfishNumberOption::Pair(_) => {
                        pair[1].carry_right(explode_result);
                    }
                }
            }
        }
    }

    pub fn carry_left(&mut self, explode_result: &mut ExplodeResult) {
        if let SnailfishNumberOption::Pair(pair) = &mut self.number {
            if let Some(left_carry) = explode_result.left_carry {
                match &pair[0].number {
                    SnailfishNumberOption::Raw(mut left_num) => {
                        println!("left carry is is{:?}", explode_result.left_carry);
                        println!("current num is: is is{:?}", left_num);

                        pair[0].number = SnailfishNumberOption::Raw(left_num + left_carry);
                        // println!("after left is{:?}", self);
                        explode_result.left_carry = None;
                    }
                    SnailfishNumberOption::Pair(_) => {
                        pair[0].carry_left(explode_result);
                    }
                }
            }
        }
    }
}
impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.number {
            SnailfishNumberOption::Raw(i) => write!(f, "{}", i.to_string()),
            SnailfishNumberOption::Pair(i) => write!(f, "{:?}", i),
        }
    }
}

fn parse_input_lines(
    raw_input_lines: &[String],
) -> Result<Vec<SnailfishNumber>, num::ParseIntError> {
    let input_lines = raw_input_lines.iter().collect::<Vec<&String>>();
    let snailfish_numbers = input_lines
        .iter()
        .map(|x| SnailfishNumber::new(x).unwrap())
        .collect::<_>();
    Ok(snailfish_numbers)
}

pub fn part_1(target_area: &Vec<SnailfishNumber>) -> i32 {
    let mut target_area_new = target_area.clone();

    let num = &mut target_area_new[0];
    println!("Before is{:?}", num);

    num.maybe_split();
    println!("After split is{:?}", num);

    let result = target_area[0].clone();
    println!("{:?}", result);
    // let x = format!("{:?}", target_area);
    // println!("In here{:?}", target_area);

    0
}

pub fn part_2(_target_area: &Vec<SnailfishNumber>) -> i32 {
    0
}

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, part_2(&encoded_data) as u64)
}

#[test]
fn test_maybe_split_true() {
    // test
    let mut num_1 = SnailfishNumber::new("[[[[[10,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    let split = num_1.maybe_split();
    assert!(split);
    assert_eq!(
        format!("{:?}", num_1),
        "[[[[[[5, 5], 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]"
    );

    let mut num_2 = SnailfishNumber::new("[[[[[11,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    let split = num_2.maybe_split();
    assert!(split);
    assert_eq!(
        format!("{:?}", num_2),
        "[[[[[[5, 6], 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]"
    );

    let mut num_3 = SnailfishNumber::new("[[[[[12,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    let split = num_3.maybe_split();
    assert!(split);
    assert_eq!(
        format!("{:?}", num_3),
        "[[[[[[6, 6], 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]]"
    );

    let mut num_4 = SnailfishNumber::new("[[[[[9,3],4],10],[7,[[8,4],9]]],[1,1]]").unwrap();
    let split = num_4.maybe_split();
    assert!(split);
    assert_eq!(
        format!("{:?}", num_4),
        "[[[[[9, 3], 4], [5, 5]], [7, [[8, 4], 9]]], [1, 1]]"
    );
}

#[test]
fn test_maybe_split_false() {
    // test
    let mut num_1 = SnailfishNumber::new("[[[[[9,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    let split = num_1.maybe_split();
    assert!(!split);
}

#[test]
fn test_maybe_explode_true() {
    // test
    let mut num_1 = SnailfishNumber::new("[[6,[5,[4,[3,2]]]],1]").unwrap();
    let exploded = num_1.maybe_explode();
    assert!(exploded);
    assert_eq!(format!("{:?}", num_1), "[[6, [5, [7, 0]]], 3]");

    let mut num_2 = SnailfishNumber::new("[7,[6,[5,[4,[3,2]]]]]").unwrap();
    let exploded = num_2.maybe_explode();
    assert!(exploded);
    assert_eq!(format!("{:?}", num_2), "[7, [6, [5, [7, 0]]]]");

    let mut num_3 = SnailfishNumber::new("[[[[[9,8],1],2],3],4]").unwrap();
    let exploded = num_3.maybe_explode();
    assert!(exploded);
    assert_eq!(format!("{:?}", num_3), "[[[[0, 9], 2], 3], 4]");

    // let mut num_4 = SnailfishNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
    // let exploded = num_4.maybe_explode();
    // assert!(exploded);
    // assert_eq!(
    //     format!("{:?}", num_4),
    //     "[[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]"
    // );
}
