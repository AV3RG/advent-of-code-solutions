use aoc_utils::num_utils::num_of_digits;

advent_of_code::solution!(7);

fn is_possible_part_1(result: &u64, nums: &[u64], achieved: u64) -> bool {
    if nums.is_empty() {
        return *result == achieved
    }
    is_possible_part_1(result, &nums[1..], achieved + nums[0]) || is_possible_part_1(result, &nums[1..], achieved * nums[0])
}

fn is_possible_part_2(result: &u64, nums: &[u64], achieved: u64) -> bool {
    if nums.is_empty() {
        return *result == achieved
    }
    is_possible_part_2(result, &nums[1..], achieved + nums[0]) || 
        is_possible_part_2(result, &nums[1..], achieved * nums[0]) || 
        is_possible_part_2(result, &nums[1..], 10_u64.pow(num_of_digits(nums[0]) as u32) * achieved + nums[0])
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed_input: Vec<(u64, Vec<u64>)> = input.trim().lines().map(|line| {
        let (result_str, nums_str) = line.trim().split_once(":").unwrap();
        let result = result_str.parse::<u64>().unwrap();
        let nums = nums_str.trim().split_whitespace().map(|part| {
            part.parse::<u64>().unwrap()
        }).collect::<Vec<_>>();
        (result, nums)
    }).collect();
    let sol = parsed_input.iter().filter(|single_input| {
        is_possible_part_1(&single_input.0, &single_input.1[1..], single_input.1[0])
    }).map(|single_input| {
        single_input.0
    }).sum();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed_input: Vec<(u64, Vec<u64>)> = input.trim().lines().map(|line| {
        let (result_str, nums_str) = line.trim().split_once(":").unwrap();
        let result = result_str.parse::<u64>().unwrap();
        let nums = nums_str.trim().split_whitespace().map(|part| {
            part.parse::<u64>().unwrap()
        }).collect::<Vec<_>>();
        (result, nums)
    }).collect();
    let sol = parsed_input.iter().filter(|single_input| {
        is_possible_part_2(&single_input.0, &single_input.1[1..], single_input.1[0])
    }).map(|single_input| {
        single_input.0
    }).sum();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
