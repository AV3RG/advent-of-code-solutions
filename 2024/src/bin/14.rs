use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use aoc_utils::grid_utils::largest_island;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x: i16,
    y: i16,
    x_velocity: i16,
    y_velocity: i16,
}

#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl Robot {

    fn simulate(&mut self, m: i16, n: i16) {
        self.x = (self.x + self.x_velocity + m) % m;
        self.y = (self.y + self.y_velocity + n) % n;
    }

    fn quadrant(&self, m: i16, n: i16) -> Option<Quadrant> {
        let to_ignore_x = if m % 2 == 1 { m / 2 } else { -1 };
        let to_ignore_y = if n % 2 == 1 { n / 2 } else { -1 };
        if self.x == to_ignore_x || self.y == to_ignore_y {
            return None;
        }
        let middle_x = m / 2;
        let middle_y = n / 2;
        if self.x < middle_x && self.y < middle_y {
            Some(Quadrant::First)
        } else if self.x >= middle_x && self.y < middle_y {
            Some(Quadrant::Second)
        } else if self.x < middle_x && self.y >= middle_y {
            Some(Quadrant::Third)
        } else {
            Some(Quadrant::Fourth)
        }
    }
}

fn make_grid(robots: &Vec<Robot>, m: i16, n: i16) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; m as usize]; n as usize];
    robots.iter().for_each(|robot| {
        grid[robot.y as usize][robot.x as usize] = '#';
    });
    grid
}

fn process_input(input: &str) -> (Vec<Robot>, i16, i16) {
    let regex = Regex::new("p=(?P<x_pos>\\d+),(?P<y_pos>\\d+) v=(?P<x_velo_sign>-?)(?P<x_velo>\\d+),(?P<y_velo_sign>-?)(?P<y_velo>\\d+)").unwrap();
    let robots = input.trim().lines().map(|line| {
        let captured = regex.captures(line).unwrap();
        let x_pos = captured["x_pos"].parse::<i16>().unwrap();
        let y_pos = captured["y_pos"].parse::<i16>().unwrap();
        let x_velo = captured["x_velo"].parse::<i16>().unwrap();
        let y_velo = captured["y_velo"].parse::<i16>().unwrap();
        let x_velo_sign = if &captured["x_velo_sign"] == "-" { -1 } else { 1 };
        let y_velo_sign = if &captured["y_velo_sign"] == "-" { -1 } else { 1 };
        Robot {
            x: x_pos,
            y: y_pos,
            x_velocity: x_velo * x_velo_sign,
            y_velocity: y_velo * y_velo_sign,
        }
    }).collect::<Vec<_>>();
    let (m, n) = (robots.iter().map(|robot| robot.x).max().unwrap() + 1, robots.iter().map(|robot| robot.y).max().unwrap() + 1);
    (robots, m, n)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut robots, m, n) = process_input(input);
    for _ in 0..100 {
        robots.iter_mut().for_each(|robot| {
            robot.simulate(m, n);
        });
    }
    let robots_filtered = robots.iter().filter_map(|robot| robot.quadrant(m, n)).collect::<Vec<_>>();
    let mut quadrant_counter = HashMap::new();
    robots_filtered.iter().for_each(|robot| {
        *quadrant_counter.entry(robot).or_insert(0) += 1;
    });
    let sol = quadrant_counter.iter().map(|(_, count)| count).product::<u32>();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut robots, m, n) = process_input(input);
    let cap = 10000;
    let mut file = File::create("output.txt").unwrap();

    for i in 1..cap {
        robots.iter_mut().for_each(|robot| {
            robot.simulate(m, n);
        });
        let grid = make_grid(&robots, m, n);
        if largest_island(&grid, m as usize, n as usize, |a| *a == '.', |a, b| a == b) < 40 { continue; }
        let visualization = grid.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
        file.write_all(("Visualization: ".to_owned() + &*i.to_string() + "\n").as_ref()).unwrap();
        file.write_all(visualization.as_bytes()).unwrap();
        file.write_all("\n\n\n".as_bytes()).unwrap();
        return Some(i);
    }
    panic!("No solution found in {} iterations", cap);
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
