use iter_tools::Itertools;
use regex::Regex;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u64> {
    let regex = Regex::new(r"(\d+)").unwrap();
    let (row, column) = regex.captures_iter(input.trim()).map(|x| x[1].parse::<u64>().unwrap() - 1).collect_tuple().unwrap();
    let diagonal = row + column;
    let mut index = ((diagonal * (diagonal + 1)) / 2) + column;
    let mut num = 20151125u64;
    while index > 0 {
        num = (num * 252533) % 33554393;
        index -= 1;
    }
    Some(num)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21629792));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
