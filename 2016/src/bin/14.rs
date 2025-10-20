use std::collections::HashMap;
use crypto::digest::Digest;
use crypto::md5::Md5;
use itertools::Itertools;

advent_of_code::solution!(14);

fn just_hash(input: &String) -> String {
    let mut md5 = Md5::new();
    md5.input(input.as_bytes());
    md5.result_str()
}

fn part_1_hash(salt: &str, index: u32, cache: &mut HashMap<u32, String>) -> String {
    if cache.contains_key(&index) {
        cache[&index].clone()
    } else {
        let md5_input = salt.to_owned() + &*index.to_string();
        let result = just_hash(&md5_input);
        cache.insert(index, result.clone());
        result
    }
}

fn stretch_hash(salt: &str, index: u32, cache: &mut HashMap<u32, String>) -> String {
    if cache.contains_key(&index) {
        cache[&index].clone()
    } else {
        let mut input = salt.to_owned() + &*index.to_string();
        for _ in 0..2017 {
            input = just_hash(&input);
        }
        cache.insert(index, input.clone());
        input
    }
}

fn find_three_of_a_kind(r: &String) -> Option<char> {
    r.chars().tuple_windows().find(|(a, b, c)| a == b && b == c).map(|(a, _, _)| a)
}

fn check_five_of_a_kind(r: &String, cc: &char) -> bool {
    r.chars().tuple_windows().any(|(a, b, c, d, e)| a == *cc && b == *cc && c == *cc && d == *cc && e == *cc)
}

fn solve(salt: &str, part1: bool) -> u32 {
    let salt = salt.trim();
    let mut cache = HashMap::new();
    let mut index = 0;
    let mut found = 0;
    while found < 64 {
        let result = if part1 {
            part_1_hash(salt, index, &mut cache)
        } else {
            stretch_hash(salt, index, &mut cache)
        };
        let three = find_three_of_a_kind(&result);
        if three.is_none() {
            index += 1;
            continue;
        }
        let c = three.unwrap();
        let mut trial = 1;
        while trial <= 1000 {
            let sub_hash = if part1 {
                part_1_hash(salt, index + trial, &mut cache)
            } else {
                stretch_hash(salt, index + trial, &mut cache)
            };
            let cc = check_five_of_a_kind(&sub_hash, &c);
            if cc {
                found += 1;
                break;
            }
            trial += 1;
        }
        index += 1;
    }
    index - 1
}

pub fn part_one(salt: &str) -> Option<u32> {
    Some(solve(salt, true))
}

pub fn part_two(salt: &str) -> Option<u32> {
    Some(solve(salt, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22728));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
