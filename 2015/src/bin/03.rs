use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos.clone());
    input.trim().chars().for_each(|c| {
        match c { 
            '>' => pos.1 += 1,
            '<' => pos.1 -= 1,
            '^' => pos.0 += 1,
            'v' => pos.0 -= 1,
            _ => panic!("Unexpected char {}", c),
        }
        visited.insert(pos.clone());
    });
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(pos1.clone());
    input.trim().chars().enumerate().for_each(|(index, c)| {
        let modifier = if index % 2 == 0 { &mut pos1 } else { &mut pos2 };
        match c {
            '>' => modifier.1 += 1,
            '<' => modifier.1 -= 1,
            '^' => modifier.0 += 1,
            'v' => modifier.0 -= 1,
            _ => panic!("Unexpected char {}", c),
        }
        visited.insert(modifier.clone());
    });
    Some(visited.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
