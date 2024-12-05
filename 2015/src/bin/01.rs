advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i16> {
    let sol = input.trim().chars().map(|c| {
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    }).sum::<i16>();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut current = 0i16;
    for (idx, c) in input.trim().chars().enumerate() {
        if c == '(' {
            current += 1;
        } else if c == ')' {
            current -= 1;
        }
        if current < 0 {
            return Some((idx as u16) + 1)
        }
    }
    panic!("Santa never enters basement!")
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
        assert_eq!(result, Some(1));
    }
}
