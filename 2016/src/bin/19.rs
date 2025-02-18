advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let num = input.trim().parse::<u64>().unwrap();
    (1 + 2 * (num - 2_u64.pow(num.ilog2()))).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let num = input.trim().parse::<u64>().unwrap();
    let power_three = 3_u64.pow(num.ilog(3));
    if num == power_three {
        num.into()
    } else if num - power_three <= power_three {
        (num - power_three).into()
    } else {
        (2 * num - 3 * power_three).into()
    }
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
        assert_eq!(result, Some(2));
    }
}
