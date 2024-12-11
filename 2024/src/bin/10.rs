use std::collections::HashSet;
use aoc_utils::grid_utils::get_manhattan_neighbours;

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct HikingStep {
    height: u8,
    reachable: HashSet<(usize, usize)>,
    paths: u64
}

impl HikingStep {

    fn new(height: u8, pos: (usize, usize)) -> HikingStep {
        HikingStep { height, reachable: if height == 9 { HashSet::from([pos]) } else { HashSet::new() }, paths: if height == 9 { 1 } else { 0 } }
    }

    fn add_neighbours(&self, grid: &mut Vec<Vec<HikingStep>>, y: usize, x: usize, n: usize, m: usize) {
        get_manhattan_neighbours((y, x), m, n)
            .iter()
            .for_each(|(neighbour_y, neighbour_x)| {
                let neighbour = &grid[*neighbour_y][*neighbour_x];
                if neighbour.height == self.height - 1 {
                    self.reachable.iter().for_each(|k| {
                        grid[*neighbour_y][*neighbour_x].reachable.insert(*k);
                    });
                    grid[*neighbour_y][*neighbour_x].paths += self.paths;
                }
            });

    }
}

fn common(input: &str) -> Vec<Vec<HikingStep>> {
    let mut grid = input.trim().lines().enumerate().map(|(y, line)| {
        line.trim().chars().enumerate().map(|(x, c)| {
            return HikingStep::new(c.to_string().parse::<u8>().unwrap(), (y, x));
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    for k in (1..=9).rev() {
        for y in 0..n {
            for x in 0..m {
                let step = grid[y][x].clone();
                if step.height != k {
                    continue;
                }
                step.add_neighbours(&mut grid, y, x, n, m);
            }
        }
    };
    grid
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = common(input);
    let sol = grid.iter().map(|line| {
        line.iter().map(|c| {
            return if c.height == 0 {
                c.reachable.len() as u32
            } else {
                0
            }
        }).sum::<u32>()
    }).sum::<u32>();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = common(input);
    let sol = grid.iter().map(|line| {
        line.iter().map(|c| {
            return if c.height == 0 {
                c.paths
            } else {
                0
            }
        }).sum::<u64>()
    }).sum::<u64>();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
