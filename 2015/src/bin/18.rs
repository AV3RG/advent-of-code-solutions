use aoc_utils::grid_utils::{get_all_neighbours};
use iter_tools::Itertools;

advent_of_code::solution!(18);

fn simulate_grid(grid: &mut Vec<Vec<u8>>, m: usize, n: usize) {
    for y in 0..n {
        for x in 0..m {
            let on_neighbours = get_all_neighbours((y, x), m, n).iter().filter(|n| grid[n.0][n.1] & 1 == 1).count();
            if grid[y][x] & 1 == 1 {
                if on_neighbours == 2 || on_neighbours == 3 {
                    grid[y][x] |= 2;
                }
            } else {
                if on_neighbours == 3 {
                    grid[y][x] |= 2;
                }
            }
        }
    }
    for y in 0..n {
        for x in 0..m {
            grid[y][x] >>= 1;
        }
    }
}

fn process_input(input: &str) -> (Vec<Vec<u8>>, usize, usize) {
    let grid = input.trim().lines().map(|line| {
        line.trim().chars().map(|c| {
            if c == '.' {
                0
            } else {
                1
            }
        }).collect_vec()
    }).collect_vec();
    let (n, m) = (grid.len(), grid[0].len());
    (grid, m, n)
}

fn turn_on_corners(grid: &mut Vec<Vec<u8>>, m: usize, n: usize) {
    grid[0][0] = 1;
    grid[0][m-1] = 1;
    grid[n-1][0] = 1;
    grid[n-1][m-1] = 1;
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, m, n) = process_input(input);
    for _ in 0..100 {
        simulate_grid(&mut grid, m, n);
    }
    grid.iter().map(|line| {
        line.iter().filter(|c| **c == 1).count() as u32
    }).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, m, n) = process_input(input);
    turn_on_corners(&mut grid, m, n);
    for _ in 0..100 {
        simulate_grid(&mut grid, m, n);
        turn_on_corners(&mut grid, m, n);
    }
    grid.iter().map(|line| {
        line.iter().filter(|c| **c == 1).count() as u32
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
