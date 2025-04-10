use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let sol = input.trim().lines().filter(|line| {
        let mut set = HashSet::new();
        for x in line.trim().split_whitespace() {
            if set.contains(x) {
                return false;
            }
            set.insert(x);
        }
        return true;
    })
    .count() as u64;
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sol = input.trim().lines().filter(|line| {
        let mut set = HashSet::new();
        for y in line.trim().split_whitespace() {
            let x = y.chars().sorted().collect::<String>();
            if set.contains(&*x) {
                return false;
            }
            set.insert(x);
        }
        return true;
    })
    .count() as u64;
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
