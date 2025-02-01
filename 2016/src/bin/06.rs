use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(6);

fn process_input(input: & str) -> Vec<HashMap<char, i32>> {
    let input = input.trim().lines().collect_vec();
    let mut commons = Vec::new();
    for _ in 0..input[0].len() {
        commons.push(HashMap::new());
    }
    input.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            *commons[i].entry(c).or_insert(0) += 1;
        })
    });
    commons
}

pub fn part_one(input: &str) -> Option<String> {
    let commons = process_input(input);
    commons.iter().map(|index| {
        let option = index.iter().sorted_by_key(|entry| -1 * entry.1).find_or_first(|entry| true);
        option.unwrap().0
    }).collect::<String>().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let commons = process_input(input);
    commons.iter().map(|index| {
        let option = index.iter().sorted_by_key(|entry| entry.1).find_or_first(|entry| true);
        option.unwrap().0
    }).collect::<String>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("easter".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("advent".to_string()));
    }
}
