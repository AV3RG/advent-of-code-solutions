use std::collections::HashMap;
use iter_tools::Itertools;

advent_of_code::solution!(9);

fn cost_mapping(input: &str) -> impl Iterator<Item = u16> {
    let mut cost_map: HashMap<String, HashMap<String, u16>> = HashMap::new();
    input.trim().lines().for_each(|line| {
        let (edge, cost_str) = line.split_once(" = ").unwrap();
        let cost = cost_str.parse::<u16>().unwrap();
        let (point1, point2) = edge.split_once(" to ").unwrap();
        cost_map.entry(point1.to_string())
            .or_insert_with(HashMap::new)
            .insert(point2.to_string(), cost);
        cost_map.entry(point2.to_string())
            .or_insert_with(HashMap::new)
            .insert(point1.to_string(), cost);
    });

    let binding = cost_map.to_owned();
    let cities = binding.keys().map(|a| a.to_owned()).collect::<Vec<String>>();
    let perms = cities.to_owned().into_iter().permutations(cities.len());

    perms.map(move |perm| {
        perm.iter()
            .tuple_windows()
            .map(|(city1, city2)| {
                cost_map[city1][city2]
            })
            .sum()
    })
}

pub fn part_one(input: &str) -> Option<u16> {
    Some(cost_mapping(input).min().unwrap())
}

pub fn part_two(input: &str) -> Option<u16> {
    Some(cost_mapping(input).max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
