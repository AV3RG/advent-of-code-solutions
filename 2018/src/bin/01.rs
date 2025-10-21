use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.trim().lines().map(|l| {
        l.parse::<i32>().unwrap()
    }).sum::<i32>())
}

pub fn part_two(input: &str) -> Option<i32> {
    let input = input.trim().lines().map(|l| {
        l.parse::<i32>().unwrap()
    }).collect_vec();
    let mut visited = HashSet::new();
    let mut current_freq = 0;
    loop {
        for f in input.iter() {
            current_freq += f;
            if visited.contains(&current_freq) {
                return Some(current_freq);
            } else {
                visited.insert(current_freq);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
