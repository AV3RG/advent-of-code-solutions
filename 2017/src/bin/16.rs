use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;

advent_of_code::solution!(16);

enum Instruction {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char)
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("s") {
            let num = s[1..].parse::<u8>().unwrap();
            Ok(Instruction::Spin(num))
        } else if s.starts_with("x") {
            let (a, b) = s[1..].split_once("/").unwrap();
            let a = a.parse::<u8>().unwrap();
            let b = b.parse::<u8>().unwrap();
            Ok(Instruction::Exchange(a, b))
        } else if s.starts_with("p") {
            let (a, b) = s[1..].split_once("/").unwrap();
            let a = a.chars().next().unwrap();
            let b = b.chars().next().unwrap();
            Ok(Instruction::Partner(a, b))
        } else {
            Err("Invalid instruction".to_string())
        }
    }
}

impl Instruction {
    fn apply(&self, store: &mut Vec<char>) {
        match self {
            Instruction::Spin(num) => {
                store.rotate_right(*num as usize);
            }
            Instruction::Exchange(a, b) => {
                store.swap(*a as usize, *b as usize);
            }
            Instruction::Partner(a, b) => {
                let a_idx = store.iter().position(|c| *c == *a).unwrap();
                let b_idx = store.iter().position(|c| *c == *b).unwrap();
                store.swap(a_idx, b_idx);
            }
        }
    }
}

fn get_store() -> Vec<char> {
    (0..16).into_iter().map(|i| {
        (i + 'a' as u8) as  char
    }).collect_vec()
}

pub fn part_one(input: &str) -> Option<String> {
    let mut store = get_store();
    input.trim().split(",").for_each(|instruction| {
        instruction.parse::<Instruction>().unwrap().apply(&mut store);
    });
    let string = store.iter().collect::<String>();
    Some(string)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut dances_left = 1_000_000_000;
    let mut store = get_store();
    let mut dance_results = HashMap::new();
    dance_results.insert(store.iter().collect::<String>(), 0);
    let runner = input.trim().split(",").map(|instruction| {
        instruction.parse::<Instruction>().unwrap()
    }).collect_vec();
    while dances_left > 0 {
        runner.iter().for_each(|instruction| {
            instruction.apply(&mut store);
        });
        dances_left -= 1;
        let current_idx = 1_000_000_000 - dances_left;
        let string = store.iter().collect::<String>();
        if dance_results.contains_key(&string) {
            let cycle_size = current_idx - dance_results[&string];
            dances_left %= cycle_size;
            let string1 = dance_results.iter().find(|p| p.1.eq(&dances_left)).unwrap().0.clone();
            return Some(string1)
        }
        dance_results.insert(string, current_idx);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("paedcbfghijklmno".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("ghidjklmnopabcef".to_string()));
    }
}
