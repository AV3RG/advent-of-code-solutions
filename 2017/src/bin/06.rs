use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(6);

fn generate_identifier_value(vec: &Vec<u8>) -> u128 {
    let mut sol = 0u128;
    vec.iter().for_each(|i| sol = (sol << 8) | (*i as u128));
    sol
}

fn redistribute_vec(vec: &mut Vec<u8>) {
    let mut idx = 0;
    let l = vec.len();
    for i in 1..l {
        if vec[i] > vec[idx] {
            idx = i;
        }
    }
    let mut to_spread = vec[idx];
    vec[idx] = 0;
    while to_spread > 0 {
        idx = (idx + 1) % l;
        vec[idx] += 1;
        to_spread -= 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut vec = input.trim().split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect_vec();
    let mut seen = HashSet::new();
    let mut count = 0;
    loop {
        let identifier = generate_identifier_value(&vec);
        if seen.contains(&identifier) {
            return Some(count);
        }
        seen.insert(identifier);
        redistribute_vec(&mut vec);
        count += 1;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut vec = input.trim().split_whitespace().map(|x| x.parse::<u8>().unwrap()).collect_vec();
    let mut seen = HashMap::new();
    let mut count = 0u64;
    loop {
        let identifier = generate_identifier_value(&vec);
        if seen.contains_key(&identifier) {
            return Some(count - seen.get(&identifier).unwrap());
        }
        seen.insert(identifier, count);
        redistribute_vec(&mut vec);
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
