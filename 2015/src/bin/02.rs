use std::cmp::min;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let sol = input.trim().lines().map(|l| {
        let mut s = l.splitn(3, "x");
        return (s.next().unwrap().parse::<u32>().unwrap(), s.next().unwrap().parse::<u32>().unwrap(), s.next().unwrap().parse::<u32>().unwrap());
    }).map(|(l, w, h)| {
        return (l * w + w * h + l * h) * 2 + min(min(l * w, w * h), l * h)
    }).sum();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sol = input.trim().lines().map(|l| {
        let mut s = l.splitn(3, "x");
        return (s.next().unwrap().parse::<u32>().unwrap(), s.next().unwrap().parse::<u32>().unwrap(), s.next().unwrap().parse::<u32>().unwrap());
    }).map(|(l, w, h)| {
        return (l * w * h) + min(min(l + w, w + h), l + h) * 2
    }).sum();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43 + 58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34 + 14));
    }
}
