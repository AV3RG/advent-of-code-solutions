use std::collections::HashSet;
use iter_tools::Itertools;

advent_of_code::solution!(19);

fn process_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (transformations, test_case) = input.trim().split_once("\n\n").unwrap();
    let transformations = transformations.trim().lines().map(|transformation| {
        let (to_replace, replace_with) = transformation.split_once(" => ").unwrap();
        (to_replace.trim(), replace_with.trim())
    }).collect_vec();
    (transformations, test_case.trim())
}

fn find_next_possibilities(transformations: &Vec<(&str, &str)>, test_case: &str) -> HashSet<String> {
    let mut possibilities = HashSet::new();
    transformations.iter().for_each(|(from, to)| {
        test_case.match_indices(from).for_each(|(i, _)| {
            possibilities.insert(test_case[..i].to_string() + to + &test_case[(i + from.len())..]);
        })
    });
    possibilities
}

fn find_previous(transformations_sorted: &Vec<(&str, &str)>, test_case: String) -> String {
    let transformation = transformations_sorted.iter().find(|(from, to)| {
        test_case.contains(to)
    }).unwrap();
    test_case.replacen(transformation.1, transformation.0, 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (transformations, test_case) = process_input(input);
    let possibilities = find_next_possibilities(&transformations, test_case);
    possibilities.len().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut path = 0;
    let (mut transformations, test_case) = process_input(input);
    transformations.sort_by_key(|tr| {
        tr.0.len() as i32 - tr.1.len() as i32
    });
    let mut current = test_case.to_string();
    while current != "e" {
        current = find_previous(&transformations, current);
        path += 1;
    };
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(535));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(212));
    }
}
