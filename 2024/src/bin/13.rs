use std::ops::Mul;
use aoc_utils::tuple_maths::{tuple_add};

advent_of_code::solution!(13);

fn capture_x_y(input: &str) -> (i64, i64) {
    let iter1 = input.split(":");
    let (x, y) = iter1.skip(1).next().unwrap().trim().split_once(", ").unwrap();
    (x[2..].parse::<i64>().unwrap(), y[2..].parse::<i64>().unwrap())
}

fn solve_buttons_brute_force(button_a: (i64, i64), button_b: (i64, i64), target: (i64, i64)) -> Option<i64> {
    let d = button_a.0 * button_b.1 - button_a.1 * button_b.0;
    if d == 0 {
        return None;
    }
    let n = (target.0 * button_b.1 - target.1 * button_b.0) / d;
    let m = (button_a.0 * target.1 - button_a.1 * target.0) / d;
    if n.mul(button_a.0) + m.mul(button_b.0) != target.0 || n.mul(button_a.1) + m.mul(button_b.1) != target.1 {
        return None;
    }
    Some(n.mul(3) + m)
}

pub fn part_one(input: &str) -> Option<i64> {
    let sol = input.trim().split("\n\n").map(|section| {
        let mut iter = section.lines();
        let button_a = capture_x_y(iter.next().unwrap());
        let button_b = capture_x_y(iter.next().unwrap());
        let target = capture_x_y(iter.next().unwrap());
        solve_buttons_brute_force(button_a, button_b, target).unwrap_or(0)
    }).sum();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sol = input.trim().split("\n\n").map(|section| {
        let mut iter = section.lines();
        let button_a = capture_x_y(iter.next().unwrap());
        let button_b = capture_x_y(iter.next().unwrap());
        let target = tuple_add(capture_x_y(iter.next().unwrap()), (10000000000000, 10000000000000));
        solve_buttons_brute_force(button_a, button_b, target).unwrap_or(0)
    }).sum();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
