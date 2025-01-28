use itertools::Itertools;

advent_of_code::solution!(3);

fn parse_line(line: &str) -> (u32, u32, u32) {
    (
        line[2..5].trim().parse::<u32>().unwrap(),
        line[7..10].trim().parse::<u32>().unwrap(),
        line[12..15].trim().parse::<u32>().unwrap()
    )
}

fn triangle_possible(a: u32, b: u32, c: u32) -> bool {
    a + b > c && b + c > a && c + a > b
}

pub fn part_one(input: &str) -> Option<usize> {
    input.lines().filter(|line| {
        let (a, b, c) = parse_line(line);
        triangle_possible(a, b, c)
    }).count().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().collect_vec().chunks(3).map(|chunk| {
        let mut counter = 0;
        let (a1, b1, c1) = parse_line(chunk[0]);
        let (a2, b2, c2) = parse_line(chunk[1]);
        let (a3, b3, c3) = parse_line(chunk[2]);
        if triangle_possible(a1, a2, a3) { counter += 1 }
        if triangle_possible(b1, b2, b3) { counter += 1 }
        if triangle_possible(c1, c2, c3) { counter += 1 }
        counter
    }).sum::<u32>().into()
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
