use std::str::FromStr;
use aoc_utils::grid_utils::{count_in_grid, print_grid};
use itertools::Itertools;

advent_of_code::solution!(8);

enum Action {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Action {

    fn run(&self, grid: &mut Vec<Vec<u8>>) {
        match self {
            Action::Rect(a, b) => {
                for y in 0..*b {
                    for x in 0..*a {
                        grid[y][x] = 1
                    }
                }
            }
            Action::RotateCol(x, shift) => {
                for y in 0..6 {
                    grid[y][*x] |= (grid[(y + 6 - shift) % 6][*x] & 1) << 1
                }
                for y in 0..6 {
                    grid[y][*x] >>= 1
                }
            }
            Action::RotateRow(y, shift) => {
                for x in 0..50 {
                    grid[*y][x] |= (grid[*y][(x + 50 - shift) % 50] & 1) << 1
                }
                for x in 0..50 {
                    grid[*y][x] >>= 1
                }
            }
        }
    }

}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("rect") {
            let (x, y) = s.split_once(" ").unwrap().1.split_once("x").unwrap();
            Ok(Action::Rect(x.parse().unwrap(), y.parse().unwrap()))
        } else if s.starts_with("rotate row") {
            let (y, shift) = s[13..].split_once(" by ").unwrap();
            Ok(Action::RotateRow(y.parse().unwrap(), shift.parse().unwrap()))
        } else if s.starts_with("rotate column") {
            let (x, shift) = s[16..].split_once(" by ").unwrap();
            Ok(Action::RotateCol(x.parse().unwrap(), shift.parse().unwrap()))
        } else {
            Err(s.to_string())
        }
    }
}

fn process_input(input: &str) -> Vec<Vec<u8>> {
    let mut main_grid = vec![vec![0u8; 50]; 6];
    input.trim().lines().for_each(|l| {
        Action::from_str(l).unwrap().run(&mut main_grid);
    });
    main_grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let main_grid = process_input(input);
    count_in_grid(&main_grid, &1).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let main_grid = process_input(input);
    main_grid.iter().for_each(|row| {
        row.iter().for_each(|&cell| {
            if cell == 1 {
                print!("X")
            } else {
                print!(" ")
            }
        });
        println!();
    });
    None
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
        assert_eq!(result, None);
    }
}
