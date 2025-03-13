use std::ops::Range;
use itertools::{sorted, Itertools};

advent_of_code::solution!(20);

fn subtract_ranges(r1: &Range<u32>, r2: &Range<u32>) -> (Option<Range<u32>>, Option<Range<u32>>) {
    let values = sorted([r1.start, r1.end, r2.start, r2.end]).collect_vec();
    let mut result = (None, None);
    if values[0] == r1.start && values[1] != r1.start {
        result.0 = Some(values[0]..values[1]);
    }
    if values[3] == r1.end && values[2] != r1.end {
        result.1 = Some(values[2]..values[3]);
    }
    result
}

fn process_subtraction(input: &str) -> Vec<Range<u32>> {
    let mut ranges = Vec::new();
    ranges.push(0..u32::MAX);
    input.trim().lines().for_each(|line| {
        let split = line.split_once("-").unwrap();
        let to_subtract = split.0.parse::<u32>().unwrap()..split.1.parse::<u32>().unwrap().checked_add(1).unwrap_or(u32::MAX);
        let mut new_ranges = Vec::new();
        ranges.iter().for_each(|range| {
            let sub_result = subtract_ranges(range, &to_subtract);
            if sub_result.0.is_some() {
                new_ranges.push(sub_result.0.unwrap());
            }
            if sub_result.1.is_some() {
                new_ranges.push(sub_result.1.unwrap());
            }
        });
        ranges = new_ranges;
    });
    ranges
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(process_subtraction(input)[0].start)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_available = process_subtraction(input);
    all_available.iter().map(|range| {
       range.end - range.start
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4294967287));
    }
}
