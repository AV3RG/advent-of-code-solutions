use std::collections::VecDeque;
use itertools::Itertools;

advent_of_code::solution!(9);

fn process_input(input: &str) -> (usize, usize) {
    let split = input.trim().split_whitespace().collect_vec();
    let players = split[0].parse::<usize>().unwrap();
    let total_turns = split[6].parse::<usize>().unwrap();
    (players, total_turns)
}


fn solve(players: usize, total_turns: usize) -> usize {
    let mut circle = VecDeque::new();
    circle.push_back(0);
    let mut player_points = vec![0; players];
    for marble in 1..=total_turns {
        let player = (marble - 1) % players;
        if marble % 23 == 0 {
            for _ in 0..7 {
                let val = circle.pop_back().unwrap();
                circle.push_front(val);
            }
            let removed = circle.pop_back().unwrap();
            player_points[player] += marble + removed;
            let val = circle.pop_front().unwrap();
            circle.push_back(val);
        } else {
            let val = circle.pop_front().unwrap();
            circle.push_back(val);
            circle.push_back(marble);
        }
    }

    *player_points.iter().max().unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (players, total_turns) = process_input(input);
    Some(solve(players, total_turns))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (players, total_turns) = process_input(input);
    Some(solve(players, total_turns * 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8317));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(74765078));
    }
}
