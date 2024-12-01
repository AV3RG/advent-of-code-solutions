use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut line1 = Vec::new();
    let mut line2 = Vec::new();
    input.trim().lines()
        .for_each(|line| {
            let split = line.trim().split_whitespace();
            split.into_iter().enumerate().for_each(|(i, c)| {
                let num = c.parse::<u32>().unwrap();
                if i == 1 {
                    line1.push(num);
                } else {
                    line2.push(num);
                }
            })
        });
    line1.sort();
    line2.sort();
    Some(line1.iter().zip(line2.iter()).map(|(a, b)| a.abs_diff(*b)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut line1 = Vec::new();
    let mut line2: HashMap<u32, u32> = HashMap::new();
    input.trim().lines()
        .for_each(|line| {
            let split = line.trim().split_whitespace();
            split.into_iter().enumerate().for_each(|(i, c)| {
                let num = c.parse::<u32>().unwrap();
                if i == 1 {
                    line1.push(num);
                } else {
                    *line2.entry(num).or_insert(0) += 1;
                }
            })
        });
    Some(
        line1.iter().map(|item| {
            item * line2.get(item).or(Some(&0)).unwrap()
        }).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
