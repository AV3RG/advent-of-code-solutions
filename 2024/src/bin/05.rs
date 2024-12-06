use std::collections::{HashMap};
use std::ops::Not;

advent_of_code::solution!(5);

struct ValidationResult {
    valid: bool,
    pair: Option<(usize, usize)>,
}

fn is_valid(vector: &Vec<u16>, validity_map: &HashMap<u16, Vec<u16>>) -> ValidationResult {
    let empty_vec: Vec<u16> = Vec::new();
    let mut line_map: HashMap<u16, usize> = HashMap::new();
    let mut valid = true;
    for (index, part_num) in vector.iter().enumerate() {
        let part_valid = validity_map.get(&part_num).unwrap_or(&empty_vec).iter().find(|k| {
            line_map.contains_key(k)
        });
        if part_valid.is_some() {
            valid = false;
            return ValidationResult {
                valid: false,
                pair: Some((line_map.get(part_valid.unwrap()).unwrap().clone(), index))
            };
            break
        }
        line_map.insert(*part_num, index);
    }
    ValidationResult { valid, pair: None }
}

pub fn part_one(input: &str) -> Option<u16> {
    let (rules_str, sequence_str) = input.trim().split_once("\n\n").unwrap();
    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();
    rules_str.trim().lines().for_each(|line| {
        let (n1, n2) = line.split_once("|").map(|(n1, n2)| {
            (n1.parse::<u16>().unwrap(), n2.parse::<u16>().unwrap())
        }).unwrap();
        rules.entry(n1.into()).or_insert_with(Vec::new).push(n2.into());
    });
    let sol = sequence_str.trim().lines().map(|line| {
        let line_nums: Vec<u16> = line.split(",").map(|num| num.parse::<u16>().unwrap()).collect();
        let result = is_valid(&line_nums, &rules);
        if !result.valid {
            return 0u16
        }
        return *line_nums.get(line_nums.len() / 2).unwrap();
    }).sum::<u16>();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u16> {
    let (rules_str, sequence_str) = input.trim().split_once("\n\n").unwrap();
    let mut rules: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut rules_reversed: HashMap<u16, Vec<u16>> = HashMap::new();
    rules_str.trim().lines().for_each(|line| {
        let (n1, n2) = line.split_once("|").map(|(n1, n2)| {
            (n1.parse::<u16>().unwrap(), n2.parse::<u16>().unwrap())
        }).unwrap();
        rules.entry(n2.into()).or_insert_with(Vec::new).push(n1.into());
        rules_reversed.entry(n1.into()).or_insert_with(Vec::new).push(n2.into());
    });
    let sol = sequence_str.trim().lines()
        .map(|line| {
            return line.split(",").map(|num| num.parse::<u16>().unwrap()).collect();
        })
        .filter_map(|line| {
            let result = is_valid(&line, &rules_reversed);
            if result.valid {
                return None;
            }
            return Some(line);
        })
        .map(|mut line| {
            let mut validation_result = is_valid(&line, &rules);
            while validation_result.valid.not() {
                let pair = validation_result.pair.unwrap();
                line.swap(pair.0, pair.1);
                validation_result = is_valid(&line, &rules);
            }
            let a = line.clone().get(line.len() / 2).unwrap().clone();
            return a;
        }).sum::<u16>();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
