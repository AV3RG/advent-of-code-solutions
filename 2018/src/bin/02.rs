use itertools::Itertools;

advent_of_code::solution!(2);

struct Wrapped {
    twice: bool,
    thrice: bool,
}

impl Wrapped {

    fn new(s: &str) -> Wrapped {

        let map = s.chars().into_group_map_by(|c| *c);
        Wrapped {
            twice: map.iter().any(|(_ ,v)| v.len() == 2),
            thrice: map.iter().any(|(_ ,v)| v.len() == 3),
        }
    }

}

pub fn part_one(input: &str) -> Option<u64> {
    let vec = input.trim().lines().map(Wrapped::new).collect_vec();
    Some(vec.iter().filter(|v| v.twice).count() as u64 * vec.iter().filter(|v| v.thrice).count() as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let (a, b) = input.trim().lines().tuple_combinations().find(|(s1, s2)| {
        s1.chars().zip(s2.chars()).filter(|(c1, c2)| c1 != c2).count() == 1
    }).unwrap();
    Some(a.chars().zip(b.chars()).filter(|(c1, c2)| c1 == c2).map(|(c1, _)| c1).collect::<String>())
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
