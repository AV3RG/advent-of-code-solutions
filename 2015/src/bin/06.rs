use std::ops::{RangeInclusive};
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(6);

struct ActionPart1 {
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    action_type: ActionTypePart1
}

struct ActionPart2 {
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    action_type: ActionTypePart2
}

enum ActionTypePart1 {
    TurnOn,
    TurnOff,
    Toggle
}

enum ActionTypePart2 {
    TurnOn,
    TurnOff,
    Toggle
}

impl ActionTypePart1 {
    fn modified_value(&self, value: bool) -> bool {
        match self {
            Self::TurnOn => true,
            Self::TurnOff => false,
            Self::Toggle => !value
        }
    }
}

impl ActionTypePart2 {
    fn modified_value(&self, value: u32) -> u32 {
        match self {
            Self::TurnOn => value + 1,
            Self::TurnOff => value.saturating_sub(1),
            Self::Toggle => value + 2
        }
    }
}

fn range_action_capture(s: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>, &str) {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<action>\w+\s?\w+) (?P<x_bottom>\d+),(?P<y_bottom>\d+) through (?P<x_top>\d+),(?P<y_top>\d+)").unwrap());
    let captured = RE.captures(s).unwrap();
    let x_bottom = captured["x_bottom"].parse::<usize>().unwrap();
    let y_bottom = captured["y_bottom"].parse::<usize>().unwrap();
    let x_top = captured["x_top"].parse::<usize>().unwrap();
    let y_top = captured["y_top"].parse::<usize>().unwrap();
    let x_range = x_bottom..=x_top;
    let y_range = y_bottom..=y_top;
    let action = captured.name("action").unwrap().as_str();
    (x_range, y_range, action)
}

impl ActionPart1 {
    fn from_str(s: &str) -> Self {
        let (x_range, y_range, action) = range_action_capture(s);
        match action {
            "turn on" => ActionPart1 { x_range, y_range, action_type: ActionTypePart1::TurnOn },
            "turn off" => ActionPart1 { x_range, y_range, action_type: ActionTypePart1::TurnOff },
            "toggle" => ActionPart1 { x_range, y_range, action_type: ActionTypePart1::Toggle },
            _ => panic!("Invalid action")
        }
    }

    fn perform(&self, grid: &mut Vec<Vec<bool>>) {
        for x in self.x_range.clone() {
            for y in self.y_range.clone() {
                grid[x][y] = self.action_type.modified_value(grid[x][y]);
            }
        }
    }
}

impl ActionPart2 {
    fn from_str(s: &str) -> Self {
        let (x_range, y_range, action) = range_action_capture(s);
        match action {
            "turn on" => ActionPart2 { x_range, y_range, action_type: ActionTypePart2::TurnOn },
            "turn off" => ActionPart2 { x_range, y_range, action_type: ActionTypePart2::TurnOff },
            "toggle" => ActionPart2 { x_range, y_range, action_type: ActionTypePart2::Toggle },
            _ => panic!("Invalid action")
        }
    }

    fn perform(&self, grid: &mut Vec<Vec<u32>>) {
        for x in self.x_range.clone() {
            for y in self.y_range.clone() {
                grid[x][y] = self.action_type.modified_value(grid[x][y]);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = vec![vec![false; 1000]; 1000];
    input.trim().lines().map(|line| {
        let action = ActionPart1::from_str(line);
        action
    }).for_each(|action| {
        action.perform(&mut grid);
    });
    let sol = grid.iter().map(|line| {
        line.iter().filter(|&&value| value).count() as u32
    }).sum();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = vec![vec![0u32; 1000]; 1000];
    input.trim().lines().map(|line| {
        let action = ActionPart2::from_str(line);
        action
    }).for_each(|action| {
        action.perform(&mut grid);
    });
    let sol = grid.iter().map(|line| {
        line.iter().sum::<u32>()
    }).sum();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(998996));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1001996));
    }
}
