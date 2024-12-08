use aoc_utils::grid_utils::{find_first_in_grid, get_horizontal_line, get_vertical_line};
use std::collections::HashSet;
use std::ops::{BitAnd, BitOrAssign, Not};

advent_of_code::solution!(6);

#[derive(Clone, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {

    fn to_bit(&self) -> u8 {
        match self {
            Direction::Left => {1}
            Direction::Right => {2}
            Direction::Up => {4}
            Direction::Down => {8}
        }
    }

    fn traverse_in_direction(&self, y: i32, x: i32) -> (i32, i32) {
        let modifier = match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        };
        (y + modifier.0, x + modifier.1)
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Left => {Direction::Up}
            Direction::Up => {Direction::Right}
            Direction::Right => {Direction::Down}
            Direction::Down => {Direction::Left}
        }
    }

}

#[derive(Clone)]
struct VisitedDirections {
    n: u8
}

impl VisitedDirections {

    fn new() -> VisitedDirections {
        VisitedDirections { n: 0 }
    }

    fn visit(&mut self, direction: &Direction) {
        self.n.bitor_assign(direction.to_bit());
    }

    fn is_visited(&self, direction: &Direction) -> bool {
        self.n.bitand(direction.to_bit()) == direction.to_bit()
    }

    fn at_all_visited(&self) -> bool {
        self.n != 0
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let vertical_range = 0..n as i32;
    let horizontal_range = 0..m as i32;
    let mut visited_with_directions = vec![vec![VisitedDirections::new(); m]; n];
    let mut current_direction = Direction::Up;
    let mut current_location = find_first_in_grid(&grid, &'^').clone().unwrap();
    let mut visited = HashSet::new();
    while visited_with_directions.get(current_location.0).unwrap().get(current_location.1).unwrap().is_visited(&current_direction).not() {
        visited_with_directions.get_mut(current_location.0).unwrap().get_mut(current_location.1).unwrap().visit(&current_direction);
        visited.insert(current_location.clone());
        let (new_y, new_x) = current_direction.traverse_in_direction(current_location.0 as i32, current_location.1 as i32);
        if vertical_range.contains(&new_y).not() || horizontal_range.contains(&new_x).not() {
            break;
        } else if grid[new_y as usize][new_x as usize] == '#' {
            current_direction = current_direction.rotate_right();
        } else {
            current_location = (new_y as usize, new_x as usize);
        }
    }
    Some(visited.len() as u32)
}


//Won't work because let's say from start we do not intentionally check for some obstacles there might be areas of the map that were never normally explored and might lead to a loop
pub fn part_two_first_try(input: &str) -> Option<u32> {
    let grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let vertical_range = 0..n as i32;
    let horizontal_range = 0..m as i32;
    let mut visited_with_directions = vec![vec![VisitedDirections::new(); m]; n];
    let mut current_direction = Direction::Up;
    let mut current_location = find_first_in_grid(&grid, &'^').clone().unwrap();
    let initial_location = current_location.clone();
    let mut points = HashSet::new();
    while visited_with_directions.get(current_location.0).unwrap().get(current_location.1).unwrap().is_visited(&current_direction).not() {
        visited_with_directions.get_mut(current_location.0).unwrap().get_mut(current_location.1).unwrap().visit(&current_direction);
        let dir_clone = current_direction.clone().rotate_right();

        let (new_y, new_x) = current_direction.traverse_in_direction(current_location.0 as i32, current_location.1 as i32);
        if vertical_range.contains(&new_y).not() || horizontal_range.contains(&new_x).not() {
            break;
        } else if grid[new_y as usize][new_x as usize] == '#' {
            current_direction = current_direction.rotate_right();
        } else {
            let old_location = current_location.clone();
            current_location = (new_y as usize, new_x as usize);
            if visited_with_directions.get(current_location.0).unwrap().get(current_location.1).unwrap().at_all_visited() {
                continue;
            }
            match dir_clone {
                Direction::Up => {
                    let vertical = &get_vertical_line(&grid, old_location.1)[..(old_location.0 + 1)];
                    let till_block = vertical.iter().enumerate().rev().take_while(|(idx, elem)| {
                        ***elem != '#'
                    }).collect::<Vec<_>>();
                    let found = till_block.iter().any(|(idx, char)| {
                        visited_with_directions.get(*idx).unwrap().get(old_location.1).unwrap().is_visited(&dir_clone)
                    });
                    if found {
                        points.insert(current_location.clone());
                    }
                },
                Direction::Down => {
                    let vertical = &get_vertical_line(&grid, old_location.1)[old_location.0..];
                    let till_block = vertical.iter().enumerate().take_while(|(idx, elem)| {
                        ***elem != '#'
                    }).collect::<Vec<_>>();
                    let found = till_block.iter().any(|(idx, char)| {
                        visited_with_directions.get(*idx + old_location.0).unwrap().get(old_location.1).unwrap().is_visited(&dir_clone)
                    });
                    if found {
                        points.insert(current_location.clone());
                    }
                },
                Direction::Left => {
                    let horizontal = &get_horizontal_line(&grid, old_location.0)[..(old_location.1 + 1)];
                    let till_block: Vec<_> = horizontal.iter().enumerate().rev().take_while(|(idx, elem)| {
                        ***elem != '#'
                    }).collect();
                    let found = till_block.iter().any(|(idx, char)| {
                        visited_with_directions.get(old_location.0).unwrap().get(*idx).unwrap().is_visited(&dir_clone)
                    });
                    if found {
                        points.insert(current_location.clone());
                    }
                },
                Direction::Right => {
                    let horizontal = &get_horizontal_line(&grid, old_location.0)[old_location.1..];
                    let till_block: Vec<_> = horizontal.iter().enumerate().take_while(|(idx, elem)| {
                        ***elem != '#'
                    }).collect();
                    let found = till_block.iter().any(|(idx, char)| {
                        visited_with_directions.get(old_location.0).unwrap().get(*idx + old_location.1).unwrap().is_visited(&dir_clone)
                    });
                    if found {
                        points.insert(current_location.clone());
                    }
                }
            }
        }
    }
    points.remove(&initial_location);
    Some(points.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let vertical_range = 0..n as i32;
    let horizontal_range = 0..m as i32;
    let mut current_location = find_first_in_grid(&grid, &'^').clone().unwrap();
    let mut current_direction = Direction::Up;
    let initial_location = current_location.clone();
    let mut count = 0;
    let mut visitable = HashSet::new();
    loop {
        visitable.insert(current_location.clone());
        let (new_y, new_x) = current_direction.traverse_in_direction(current_location.0 as i32, current_location.1 as i32);
        if vertical_range.contains(&new_y).not() || horizontal_range.contains(&new_x).not() {
            break;
        } else if grid[new_y as usize][new_x as usize] == '#' {
            current_direction = current_direction.rotate_right();
        } else {
            current_location = (new_y as usize, new_x as usize);
        }
    }
    visitable.remove(&initial_location);
    vertical_range.clone().for_each(|y| {
        horizontal_range.clone().into_iter().for_each(|x| {
            if visitable.contains(&(y as usize, x as usize)).not() {
                return;
            }
            grid[y as usize][x as usize] = '#';
            current_location = initial_location;
            current_direction = Direction::Up;
            let mut visited_with_directions = vec![vec![VisitedDirections::new(); m]; n];
            let mut out_of_bounds = false;
            while visited_with_directions.get(current_location.0).unwrap().get(current_location.1).unwrap().is_visited(&current_direction).not() {
                visited_with_directions[current_location.0][current_location.1].visit(&current_direction);
                let (new_y, new_x) = current_direction.traverse_in_direction(current_location.0 as i32, current_location.1 as i32);
                if vertical_range.contains(&new_y).not() || horizontal_range.contains(&new_x).not() {
                    out_of_bounds = true;
                    break;
                } else if grid[new_y as usize][new_x as usize] == '#' {
                    current_direction = current_direction.rotate_right();
                } else {
                    current_location = (new_y as usize, new_x as usize);
                }
            }
            if !out_of_bounds {
                count += 1
            }
            grid[y as usize][x as usize] = '.';
        })
    });
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
