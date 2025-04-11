use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut vec = input.trim().lines().map(|l| l.parse::<i32>().unwrap()).collect_vec();
    let mut pointer = 0i32;
    let mut steps = 0;
    while pointer >= 0 && (pointer as usize) < vec.len() {
        vec[pointer as usize] += 1;
        steps += 1;
        pointer += vec[pointer as usize] - 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut vec = input.trim().lines().map(|l| l.parse::<i32>().unwrap()).collect_vec();
    let mut pointer = 0i32;
    let mut steps = 0;
    while pointer >= 0 && (pointer as usize) < vec.len() {
        if vec[pointer as usize] < 3 {
            vec[pointer as usize] += 1;
            pointer += vec[pointer as usize] - 1;
        } else {
            vec[pointer as usize] -= 1;
            pointer += vec[pointer as usize] + 1;
        }
        steps += 1;
    }
    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
