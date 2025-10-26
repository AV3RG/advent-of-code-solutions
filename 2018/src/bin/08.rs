use std::collections::VecDeque;
use itertools::Itertools;

advent_of_code::solution!(8);

fn find_next_1(vec: &mut VecDeque<u32>) -> u32 {
    if vec.len() == 0 {
        return 0;
    }
    let mut sum = 0;
    let child_nodes = vec.pop_front().unwrap();
    let meta = vec.pop_front().unwrap();
    for _ in 0..child_nodes {
        sum += find_next_1(vec);
    }
    for _ in 0..meta {
        sum += vec.pop_front().unwrap();
    }
    sum
}

fn find_next_2(vec: &mut VecDeque<u32>) -> u32 {
    if vec.len() == 0 {
        return 0;
    }
    let mut sum = 0;
    let child_nodes = vec.pop_front().unwrap();
    let meta = vec.pop_front().unwrap();
    if child_nodes == 0 {
        for _ in 0..meta {
            sum += vec.pop_front().unwrap();
        }
        return sum;
    }
    let mut child_node_values = Vec::new();
    for _ in 0..child_nodes {
        child_node_values.push(find_next_2(vec));
    }
    for _ in 0..meta {
        let child_node = vec.pop_front().unwrap();
        if child_node == 0 {
            continue;
        }
        sum += child_node_values.get(child_node as usize - 1).unwrap_or(&0);
    }
    return sum;

    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let vec = input.trim().split_whitespace().map(|t| t.parse::<u32>().unwrap()).collect_vec();
    let mut vec = VecDeque::from(vec);
    let sol = find_next_1(&mut vec);
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = input.trim().split_whitespace().map(|t| t.parse::<u32>().unwrap()).collect_vec();
    let mut vec = VecDeque::from(vec);
    let sol = find_next_2(&mut vec);
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(66));
    }
}
