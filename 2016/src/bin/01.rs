use std::collections::HashSet;
use aoc_utils::tuple_maths::{tuple_add, tuple_scalar_multiply};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut curr_pos = (0, 0);
    let mut dir_vector = (0, 1);
    input.trim().split(", ").for_each(|move_spec| {
        let dir = &move_spec[0..1];
        if dir == "R" {
            dir_vector = (dir_vector.1 * 1, dir_vector.0 * -1);
        } else if dir == "L" {
            dir_vector = (dir_vector.1 * -1, dir_vector.0 * 1);
        }
        let to_add = tuple_scalar_multiply(dir_vector, move_spec[1..].parse::<i32>().unwrap());
        curr_pos = tuple_add(curr_pos, to_add);
    });
    (curr_pos.0.abs() + curr_pos.1.abs()).into()
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut curr_pos = (0i32, 0i32);
    let mut dir_vector = (0i32, 1i32);
    let mut visited = HashSet::new();
    visited.insert(curr_pos.clone());
    for move_spec in input.trim().split(", ") {
        let dir = &move_spec[0..1];
        if dir == "R" {
            dir_vector = (dir_vector.1 * 1, dir_vector.0 * -1);
        } else if dir == "L" {
            dir_vector = (dir_vector.1 * -1, dir_vector.0 * 1);
        }
        for _ in 0..move_spec[1..].parse::<i32>().unwrap() {
            curr_pos = tuple_add(curr_pos, dir_vector);
            if visited.contains(&curr_pos) {
                return (curr_pos.0.abs() + curr_pos.1.abs()).into()
            }
            visited.insert(curr_pos.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
