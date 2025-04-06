use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    input.trim()
        .chars()
        .collect_vec()
        .into_iter()
        .circular_tuple_windows()
        .filter(|&(a, b)| a == b)
        .map(|(a, _b)| ((a as u8) - 48u8) as u64)
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let chars = input.trim().chars().collect_vec();
    let len = chars.len();
    let mut sol = 0;
    for i in 0..len {
        if chars[i] == chars[(i + (len / 2)) % len] {
            sol += ((chars[i] as u8) - 48u8) as u64
        }
    }
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
