use itertools::Itertools;

advent_of_code::solution!(16);

fn generate_next(current: &String) -> String {
    let mut soln = current.clone();
    soln.push('0');
    soln.push_str(&*current.chars().rev().map(|c| {
        if c == '0' { '1' } else { '0' }
    }).collect::<String>());
    soln
}

fn generate_checksum(case: &String) -> String {
    case.chars().chunks(2).into_iter().map(|c| {
        let mut chunk_iter = c.into_iter();
        let a = chunk_iter.next().unwrap();
        let b = chunk_iter.next().unwrap();
        return if a == b {
            '1'
        } else {
            '0'
        }
    }).collect::<String>()
}

fn run_input(input: &str, length: usize) -> String {
    let mut current = input.trim().to_string();
    while current.len() < length {
        current = generate_next(&current);
    }
    current = current[..length].parse().unwrap();
    current = generate_checksum(&current);
    while current.len() % 2 == 0 {
        current = generate_checksum(&current);
    }
    current.into()
}

pub fn part_one(input: &str) -> Option<String> {
    run_input(input, 272).into()
}

pub fn part_two(input: &str) -> Option<String> {
    run_input(input, 35651584).into()
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
