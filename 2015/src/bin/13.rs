use std::collections::HashMap;
use iter_tools::Itertools;

advent_of_code::solution!(13);

fn process_input(input: &str) -> (HashMap<(usize, usize), i32>, usize) {
    let processed_differences = input.trim().lines().map(|line| {
        let mut parts = line.split_whitespace().into_iter();
        parts.next();
        parts.next();
        let sign = if parts.next().unwrap() == "lose" { -1 } else { 1 };
        let quantity = parts.next().unwrap().parse::<i32>().unwrap();
        sign * quantity
    }).collect_vec();
    let mut count = 2;
    while count * (count - 1) != processed_differences.len() {
        count += 1;
    }
    let mut result = HashMap::new();
    for i in 0..count {
        for j in 0..i {
            result.insert((i, j), processed_differences[i * (count - 1) + j]);
        }
        for j in i + 1..count {
            result.insert((i, j), processed_differences[i * (count - 1) + j - 1]);
        }
    }
    (result, count)
}

fn calculate_score(differences: &HashMap<(usize, usize), i32>, permutation: &Vec<&usize>) -> i32 {
    permutation.windows(2).map(|pair| {
        differences[&(*pair[0], *pair[1])] + differences[&(*pair[1], *pair[0])]
    }).sum::<i32>() + differences[&(*permutation[0], *permutation[permutation.len() - 1])] + differences[&(*permutation[permutation.len() - 1], *permutation[0])]
}

fn find_max(differences: &HashMap<(usize, usize), i32>, count: usize) -> Option<i32> {
    let count_vec = (0..count).collect_vec();
    count_vec.iter().permutations(count).map(|permutation| {
        calculate_score(&differences, &permutation)
    }).max()
}

pub fn part_one(input: &str) -> Option<i32> {
    let (differences, count) = process_input(input);
    find_max(&differences, count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (mut differences, mut count) = process_input(input);
    for i in 0..count {
        differences.insert((i, count), 0);
        differences.insert((count, i), 0);
    }
    count += 1;
    find_max(&differences, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
