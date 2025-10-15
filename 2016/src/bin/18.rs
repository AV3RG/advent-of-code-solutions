use aoc_utils::bit_utils::count_bits;

advent_of_code::solution!(18);

fn get_next_num(num: &u128, and_mask: &u128) -> u128 {
    ((num << 1) ^ (num >> 1)) & and_mask
}

fn process(input: &str, total_num: u128) -> u128 {
    let mut num = 0u128;
    input.trim().chars().for_each(|c| {
        if c == '.' {
            num <<= 1;
        } else {
            num = (num << 1) | 1;
        }
    });
    let mut unsafe_count = 0;
    let and_mask = 2u128.pow(input.trim().len() as u32) - 1;
    unsafe_count += count_bits(&num);
    for _ in 1..total_num {
        let new_num = get_next_num(&num, &and_mask);
        unsafe_count += count_bits(&new_num);
        num = new_num;
    }
    total_num * input.trim().len() as u128 - unsafe_count
}

pub fn part_one(input: &str) -> Option<u128> {
    Some(process(input, 40))
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(process(input, 400000))
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
