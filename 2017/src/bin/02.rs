use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    input.trim().lines().map(|line| {
        let split = line.split_whitespace().map(|p| p.parse::<u32>().unwrap()).collect_vec();
        split.iter().minmax().into_option().map(|(min, max)| max - min).unwrap()
    }).sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.trim().lines().map(|line| {
        let split = line.split_whitespace().map(|p| p.parse::<u32>().unwrap()).collect_vec();
        split.iter()
            .tuple_combinations()
            .find(|&(a, b)| { a % b == 0 || b % a == 0 })
            .map(|(a, b)| if a > b { a / b } else { b / a })
            .unwrap()
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9));
    }
}
