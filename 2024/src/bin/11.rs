use std::collections::{HashMap};
use std::ops::Not;
use aoc_utils::num_utils::{first_n_digits, last_n_digits, num_of_digits};

advent_of_code::solution!(11);

enum HistorianResult {
    Single(u64),
    Double(u64, u64)
}

impl HistorianResult {
    fn add_to_map(&self, map: &mut HashMap<u64, u64>, times: &u64) {
        match self {
            HistorianResult::Single(val) => {
                *map.entry(*val).or_insert(0) += times;
            }
            HistorianResult::Double(val1, val2) => {
                *map.entry(*val1).or_insert(0) += times;
                *map.entry(*val2).or_insert(0) += times;
            }
        }
    }

    fn calculate_result(num: u64) -> HistorianResult {
        let digits = num_of_digits(num);
        if num == 0 {
            HistorianResult::Single(1)
        }
        else if digits % 2 == 0 {
            HistorianResult::Double(first_n_digits(num, digits / 2), last_n_digits(num, digits / 2))
        } else {
            HistorianResult::Single(num * 2024)
        }
    }
}

fn simulate(input: &str, iterations: u8) -> HashMap<u64, u64> {
    let mut result_cache: HashMap<u64, HistorianResult> = HashMap::new();
    let mut counter_state: HashMap<u64, u64> = HashMap::new();
    input.trim()
        .split_whitespace()
        .for_each(|c| {
            *counter_state.entry(c.to_string().parse::<u64>().unwrap()).or_insert(0) += 1
        });
    for _ in 0..iterations {
        let mut new_state: HashMap<u64, u64> = HashMap::new();
        counter_state.iter().for_each(|(val, count)| {
            if result_cache.contains_key(val).not() {
                result_cache.insert(*val, HistorianResult::calculate_result(*val));
            }
            result_cache.get(val).unwrap().add_to_map(&mut new_state, count);
        });
        counter_state = new_state;
    }
    counter_state
}

pub fn part_one(input: &str) -> Option<u64> {
    let state = simulate(input, 25);
    Some(state.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let state = simulate(input, 75);
    Some(state.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
