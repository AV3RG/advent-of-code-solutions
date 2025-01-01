use aoc_utils::grid_utils::get_manhattan_neighbours;
use pathfinding::prelude::dijkstra;

advent_of_code::solution!(18);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct GridPos(usize, usize);

impl GridPos {
    fn successors(&self, size: usize, bytes: &Vec<GridPos>, simulate_bytes_top: usize) -> Vec<GridPos> {
        get_manhattan_neighbours((self.0, self.1), size, size)
            .into_iter()
            .filter(|a| {
                !bytes.iter().position(|l| l.0 == a.0 && l.1 == a.1).is_some_and(|f| f < simulate_bytes_top)
            })
            .map(|k| GridPos(k.0, k.1))
            .collect::<Vec<_>>()
    }
}

fn process_input(input: &str) -> (Vec<GridPos>, usize, usize) {
    let bytes = input.trim()
        .lines()
        .map(|a| {
            let(x, y) = a.split_once(",").unwrap();
            return GridPos(y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    let size = bytes.iter().map(|a| a.0).max().unwrap() + 1;
    let simulate_bytes = if size == 7 { 12 } else { 1024 };
    (bytes, size, simulate_bytes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (bytes, size, simulate_bytes) = process_input(input);
    let sol = dijkstra(
        &GridPos(0, 0), 
        |p| p.successors(size, &bytes, simulate_bytes).into_iter().map(|p| (p, 1)).collect::<Vec<_>>(),
        |k| k.0 == size - 1 && k.1 == size - 1
    );
    Some(sol.unwrap().1)
}

pub fn part_two(input: &str) -> Option<String> {
    let (bytes, size, mut left) = process_input(input);
    let mut right = bytes.len();
    let start = GridPos(0, 0);
    while left <= right {
        let middle = (left + right) / 2;
        let sol = dijkstra(
            &start,
            |p| p.successors(size, &bytes, middle).into_iter().map(|p| (p, 1)).collect::<Vec<_>>(),
            |k| k.0 == size - 1 && k.1 == size - 1
        );
        if sol.is_some() {
            left = middle + 1;
        } else {
            right = middle - 1
        }
    }
    let sol = &bytes[left.max(right) - 1];
    Some(sol.1.to_string() + "," + &*sol.0.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
