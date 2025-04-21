use std::collections::HashSet;
use itertools::Itertools;

advent_of_code::solution!(12);

fn process_input(input: &str) -> Vec<(u16, Vec<u16>)> {
    input.trim().lines().map(|line| {
        let (root, targets) = line.split_once("<->").unwrap();
        let root = root.trim().parse::<u16>().unwrap();
        let targets = targets.trim().split(",").map(|t| t.trim().parse::<u16>().unwrap()).collect_vec();
        (root, targets)
    }).collect_vec()
}

fn create_group(input: &Vec<(u16, Vec<u16>)>, starting_node: u16) -> HashSet<u16> {
    let mut to_visit = vec![starting_node];
    let mut visited = HashSet::new();
    while !to_visit.is_empty() {
        let next = to_visit.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);
        to_visit.extend(input[next as usize].1.iter());
    }
    visited
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = process_input(input);
    let visited = create_group(&input, 0);
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<u16> {
    let input = process_input(input);
    let mut initial_visited: HashSet<u16> = HashSet::new();
    let mut num_groups = 0;
    for i in 0..input.len() {
        if initial_visited.contains(&(i as u16)) { continue; }
        num_groups += 1;
        let new_visited = create_group(&input, i as u16);
        initial_visited.extend(new_visited.iter());
    }
    Some(num_groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
