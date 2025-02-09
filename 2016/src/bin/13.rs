use std::collections::{HashMap, HashSet, VecDeque};
use aoc_utils::grid_utils::get_manhattan_neighbours;
use itertools::Itertools;
use pathfinding::prelude::bfs;

advent_of_code::solution!(13);

fn is_open_space(y: usize, x: usize, fav_number: usize) -> bool {
    let mut num = (x*x + 3*x + 2*x*y + y + y*y) + fav_number;
    let mut bits = true;
    while num > 0 {
        if num & 1 == 1 { bits = !bits }
        num >>= 1;
    }
    bits
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let fav_number = input.trim().parse::<usize>().unwrap();
    cache.insert((1, 1), true);
    let solution_path = bfs(
        &(1, 1),
        |curr| {
            get_manhattan_neighbours(curr.clone(), 10000, 10000).into_iter().filter(|neigh| {
                if !cache.contains_key(neigh) {
                    cache.insert(neigh.clone().to_owned(), is_open_space(neigh.0, neigh.1, fav_number));
                }
                return *cache.get(neigh).unwrap()
            }).collect_vec()
        },
        |end| {
            return end.1 == 31 && end.0 == 39
        }
    ).unwrap();
    (solution_path.len() - 1).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut pending = VecDeque::new();
    let mut open_cache = HashMap::new();
    let fav_number = input.trim().parse::<usize>().unwrap();
    visited.insert((1, 1));
    pending.push_back(((1, 1), 0));
    while !pending.is_empty() {
        let (curr, dist) = pending.pop_front().unwrap();
        visited.insert(curr);
        if dist == 50 {
            continue;
        }
        get_manhattan_neighbours(curr.clone(), 10000, 10000).into_iter().for_each(|neigh| {
            if visited.contains(&neigh) {
               return;
            }
            if !open_cache.contains_key(&neigh) {
                open_cache.insert(neigh.clone(), is_open_space(neigh.0, neigh.1, fav_number));
            }
            if *open_cache.get(&neigh).unwrap() {
                if (neigh.0 + neigh.1) <= 52 {
                    pending.push_back((neigh, dist + 1));
                }
            }
        })
    };
    visited.len().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
