// use std::borrow::Borrow;
use std::fmt;
use std::num;

const MAX_RAW_VALUE: i32 = 9;

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
        // line: [[[[4,3],4],4],[7,[[8,4],9]]]

        // ok check ends and starts with []
        assert!(line.starts_with('[') && line.ends_with(']'));

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
        let left_num = if left_num_str.starts_with('[') {
            Box::new(SnailfishNumber::new(left_num_str).unwrap())
        } else {
            Box::new(SnailfishNumber {
                number: SnailfishNumberOption::Raw(left_num_str.parse::<i32>().unwrap()),
            })
        };

        let right_num_str = &line[middle_comma_index.unwrap() + 1..line.len()];
        let right_num = if right_num_str.starts_with('[') {
            Box::new(SnailfishNumber::new(right_num_str).unwrap())
        } else {
            Box::new(SnailfishNumber {
                number: SnailfishNumberOption::Raw(right_num_str.parse::<i32>().unwrap()),
            })
        };

        let number_pair = vec![left_num, right_num];

        Ok(SnailfishNumber {
            number: SnailfishNumberOption::Pair(number_pair),
        })
    }
    pub fn add(mut self, num_to_add: &SnailfishNumber) -> SnailfishNumber {
        // e.g. [1,2] + [[3,4],5] = [[1,2],[[3,4],5]]
        self = SnailfishNumber {
            number: SnailfishNumberOption::Pair(vec![Box::new(self), Box::new(num_to_add.clone())]),
        };
        self.reduce();
        self
    }
    pub fn reduce(&mut self) {
        // Try explode, then try split, then repeat.
        loop {
            if !self.maybe_explode() && !self.maybe_split() {
                // No explode or split - can't be reduced anymore
                break;
            }
        }
    }
    pub fn maybe_split(&mut self) -> bool {
        match &mut self.number {
            SnailfishNumberOption::Raw(raw_value) => {
                if *raw_value > MAX_RAW_VALUE {
                    let new_left_num = Box::new(SnailfishNumber {
                        number: SnailfishNumberOption::Raw(
                            ((*raw_value as f32) / 2_f32).floor() as i32
                        ),
                    });
                    let new_right_num = Box::new(SnailfishNumber {
                        number: SnailfishNumberOption::Raw(
                            ((*raw_value as f32) / 2_f32).ceil() as i32
                        ),
                    });
                    let new_pair = vec![new_left_num, new_right_num];

                    self.number = SnailfishNumberOption::Pair(new_pair);
                    true
                } else {
                    false
                }
            }
            SnailfishNumberOption::Pair(pair) => pair[0].maybe_split() || pair[1].maybe_split(),
        }
    }
    pub fn maybe_explode(&mut self) -> bool {
        // Return true if explode happened, false otherwise
        let explode_result = ExplodeResult {
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
            if depth == 4 {
                // Need to explode this value
                explode_result.exploded = true;
                if let SnailfishNumberOption::Raw(left_num) = pair[0].number {
                    explode_result.left_carry = Some(left_num);
                } else {
                    panic!("Unexpected - left num is pair");
                }
                if let SnailfishNumberOption::Raw(right_num) = pair[1].number {
                    explode_result.right_carry = Some(right_num);
                } else {
                    panic!("Unexpected - right num is pair");
                }
                self.number = SnailfishNumberOption::Raw(0);
            } else {
                // Check if left needs to explode
                explode_result = pair[0].maybe_do_explode(depth + 1, explode_result);
                if explode_result.exploded {
                    self.carry_right(&mut explode_result, 1);
                } else {
                    // Check if right needs to explode
                    explode_result = pair[1].maybe_do_explode(depth + 1, explode_result);
                    if explode_result.exploded {
                        self.carry_left(&mut explode_result, 0);
                    }
                }
            }
        }

        explode_result
    }

    pub fn carry_right(&mut self, explode_result: &mut ExplodeResult, index: usize) {
        // try to refactor to remove index, but doing the pair stuff in the calling function
        if let SnailfishNumberOption::Pair(pair) = &mut self.number {
            if let Some(right_carry) = explode_result.right_carry {
                match &pair[index].number {
                    SnailfishNumberOption::Raw(right_num) => {
                        pair[index].number = SnailfishNumberOption::Raw(right_num + right_carry);
                        explode_result.right_carry = None;
                    }
                    SnailfishNumberOption::Pair(_) => {
                        pair[index].carry_right(explode_result, 0);
                    }
                }
            }
        }
    }

    pub fn carry_left(&mut self, explode_result: &mut ExplodeResult, index: usize) {
        if let SnailfishNumberOption::Pair(pair) = &mut self.number {
            if let Some(left_carry) = explode_result.left_carry {
                match &pair[index].number {
                    SnailfishNumberOption::Raw(left_num) => {
                        pair[index].number = SnailfishNumberOption::Raw(left_num + left_carry);
                        explode_result.left_carry = None;
                    }
                    SnailfishNumberOption::Pair(_) => {
                        pair[index].carry_left(explode_result, 1);
                    }
                }
            }
        }
    }

    pub fn magnitude(&self) -> i32 {
        match &self.number {
            SnailfishNumberOption::Raw(raw_num) => *raw_num,
            SnailfishNumberOption::Pair(pair) => {
                (pair[0].magnitude() * 3) + (pair[1].magnitude() * 2)
            }
        }
    }
}
impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.number {
            SnailfishNumberOption::Raw(i) => write!(f, "{}", i),
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

pub fn part_1(numbers: &[SnailfishNumber]) -> i32 {
    let my_numbers = numbers.to_owned();

    let sum = my_numbers
        .into_iter()
        .reduce(|sum, number| sum.add(&number))
        .unwrap();

    sum.magnitude()
}

pub fn part_2(numbers: &[SnailfishNumber]) -> i32 {
    let my_numbers = numbers.to_owned();

    let mut max = 0;

    for first_num in my_numbers.iter() {
        for second_num in my_numbers.iter() {
            let first_num = first_num.clone();
            let second_num = second_num.clone();

            let result = first_num.add(&second_num).magnitude();

            if result > max {
                max = result;
            }
        }
    }

    max
}

pub fn day18(input_lines: &[String]) -> (u64, u64) {
    let encoded_data = parse_input_lines(input_lines).unwrap_or_else(|err| {
        panic!("Got error : {} , when trying to parse the input lines", err);
    });
    (part_1(&encoded_data) as u64, part_2(&encoded_data) as u64)
}

#[test]
fn test_maybe_split_true() {
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
    let mut num_1 = SnailfishNumber::new("[[[[[9,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
    let split = num_1.maybe_split();
    assert!(!split);
}

#[test]
fn test_maybe_explode_true() {
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

    let mut num_4 = SnailfishNumber::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
    let exploded = num_4.maybe_explode();
    assert!(exploded);
    assert_eq!(
        format!("{:?}", num_4),
        "[[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]"
    );

    let mut num_5 = SnailfishNumber::new("[[[[[9,8],1],2],3],4]").unwrap();
    let exploded = num_5.maybe_explode();
    assert!(exploded);
    assert_eq!(format!("{:?}", num_5), "[[[[0, 9], 2], 3], 4]");

    let mut num_6 = SnailfishNumber::new("[[6,[5,[4,[3,2]]]],1]").unwrap();
    let exploded = num_6.maybe_explode();
    assert!(exploded);
    assert_eq!(format!("{:?}", num_6), "[[6, [5, [7, 0]]], 3]");

    let mut num_7 = SnailfishNumber::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();
    let exploded = num_7.maybe_explode();
    assert!(exploded);
    assert_eq!(
        format!("{:?}", num_7),
        "[[3, [2, [8, 0]]], [9, [5, [7, 0]]]]"
    );
}

#[test]
fn test_add() {
    let mut num_1 = SnailfishNumber::new("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
    let num_2 = SnailfishNumber::new("[1,1]").unwrap();
    num_1 = num_1.add(&num_2);
    assert_eq!(
        format!("{:?}", num_1),
        "[[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]"
    );
}

#[test]
fn test_magnitude() {
    let num_1 = SnailfishNumber::new("[9,1]").unwrap();
    assert_eq!(num_1.magnitude(), 29);

    let num_2 = SnailfishNumber::new("[1,9]").unwrap();
    assert_eq!(num_2.magnitude(), 21);

    let num_3 = SnailfishNumber::new("[[9,1],[1,9]]").unwrap();
    assert_eq!(num_3.magnitude(), 129);

    let num_4 =
        SnailfishNumber::new("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
            .unwrap();
    assert_eq!(num_4.magnitude(), 4140);
}
