use std::str::FromStr;
use itertools::Itertools;

advent_of_code::solution!(21);

enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnChar(char),
    ReversePositions(usize, usize),
    MovePos(usize, usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect_vec();
        if s.starts_with("swap position") {
            Ok(Instruction::SwapPos(split[2].parse().unwrap(), split[5].parse().unwrap()))
        } else if s.starts_with("swap letter") {
            Ok(Instruction::SwapLetter(split[2].chars().next().unwrap(), split[5].chars().next().unwrap()))
        } else if s.starts_with("rotate left") {
            Ok(Instruction::RotateLeft(split[2].parse().unwrap()))
        } else if s.starts_with("rotate right") {
            Ok(Instruction::RotateRight(split[2].parse().unwrap()))
        } else if s.starts_with("rotate based on") {
            Ok(Instruction::RotateBasedOnChar(split.last().unwrap().chars().next().unwrap()))
        } else if s.starts_with("reverse positions") {
            Ok(Instruction::ReversePositions(split[2].parse().unwrap(), split[4].parse().unwrap()))
        } else if s.starts_with("move position") {
            Ok(Instruction::MovePos(split[2].parse().unwrap(), split[5].parse().unwrap()))
        } else {
            Err(format!("Unknown instruction: {}", s))
        }
    }
}

impl Instruction {
    fn execute(&self, word: &mut Vec<char>) {
        let l = word.len();
        match self {
            Instruction::SwapPos(a, b) => {
                word.swap(*a, *b);
            }
            Instruction::SwapLetter(a, b) => {
                let index_a = word.iter().position(|&c| c == *a).unwrap();
                let index_b = word.iter().position(|&c| c == *b).unwrap();
                word.swap(index_a, index_b);
            }
            Instruction::RotateLeft(n) => {
                word.rotate_left(*n % l);
            }
            Instruction::RotateRight(n) => {
                word.rotate_right(*n % l);
            }
            Instruction::RotateBasedOnChar(c) => {
                let index = word.iter().position(|&cc| cc == *c).unwrap();
                let add = if index >=4 { 2 } else { 1 };
                word.rotate_right((index + add) % l);
            }
            Instruction::ReversePositions(a, b) => {
                let mut left = *a;
                let mut right = *b;
                while left < right {
                    word.swap(left, right);
                    left += 1;
                    right -= 1;
                }
            }
            Instruction::MovePos(a, b) => {
                let char = word.get(*a).unwrap().clone();
                word.remove(*a);
                word.insert(*b, char)
            }
        }
    }

    fn execute_reverse(&self, word: &mut Vec<char>) {
        let l = word.len();
        match self {
            Instruction::SwapPos(_, _) | Instruction::SwapLetter(_, _) | Instruction::ReversePositions(_, _) => {
                self.execute(word);
            }
            Instruction::RotateLeft(n) => {
                Instruction::RotateRight(*n).execute(word);
            }
            Instruction::RotateRight(n) => {
                Instruction::RotateLeft(*n).execute(word)
            }
            Instruction::RotateBasedOnChar(_) => {
                for i in 0..l {
                    let mut vec = word.clone();
                    vec.rotate_left(i);
                    self.execute(&mut vec);
                    if word.clone().eq(&vec) {
                        word.rotate_left(i);
                        break;
                    }
                }
            }
            Instruction::MovePos(a, b) => {
                Instruction::MovePos(*b, *a).execute(word);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut word = "abcdefgh".chars().collect_vec();
    input.trim().lines().for_each(|line| {
        line.parse::<Instruction>().unwrap().execute(&mut word);
    });
    Some(word.iter().collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut word = "fbgdceah".chars().collect_vec();
    input.trim().lines().rev().for_each(|line| {
        line.parse::<Instruction>().unwrap().execute_reverse(&mut word);
    });
    Some(word.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("fbdecgha".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("efghdabc".to_string()));
    }
}
