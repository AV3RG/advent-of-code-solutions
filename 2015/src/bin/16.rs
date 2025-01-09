use std::collections::HashMap;
use std::str::FromStr;
use iter_tools::Itertools;
use strum_macros::EnumString;

advent_of_code::solution!(16);

#[derive(EnumString, Eq, PartialEq, Hash)]
#[strum(serialize_all = "lowercase")]
enum SueProperty {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

fn acceptable_part_1(to_test: &HashMap<SueProperty, u16>, match_spec: &HashMap<SueProperty, u16>) -> bool {
    to_test.iter().all(|entry| {
        match_spec[entry.0] == *entry.1
    })
}

fn acceptable_part_2(to_test: &HashMap<SueProperty, u16>, match_spec: &HashMap<SueProperty, u16>) -> bool {
    to_test.iter().all(|entry| {
        match entry.0 {
            SueProperty::Cats | SueProperty::Trees => entry.1 > &match_spec[entry.0],
            SueProperty::Pomeranians | SueProperty::Goldfish => entry.1 < &match_spec[entry.0],
            _ => entry.1 == &match_spec[entry.0]
        }
    })
}

fn process_input(input: &str) -> Vec<HashMap<SueProperty, u16>> {
    input.trim().lines().map(|line| {
        let mut props = line.split_once(": ").unwrap().1.split(", ").into_iter().peekable();
        let mut map = HashMap::new();
        while props.peek().is_some() {
            let (name, value) = props.next().unwrap().split_once(": ").unwrap();
            map.insert(SueProperty::from_str(name).unwrap(), value.parse::<u16>().unwrap());
        }
        map
    }).collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let aunt_list = process_input(input);
    let match_spec = HashMap::from([
        (SueProperty::Children, 3),
        (SueProperty::Cats, 7),
        (SueProperty::Samoyeds, 2),
        (SueProperty::Pomeranians, 3),
        (SueProperty::Akitas, 0),
        (SueProperty::Vizslas, 0),
        (SueProperty::Goldfish, 5),
        (SueProperty::Trees, 3),
        (SueProperty::Cars, 2),
        (SueProperty::Perfumes, 1)
    ]);
    aunt_list.iter()
        .position(|test| acceptable_part_1(test, &match_spec))
        .map(|i| i as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let aunt_list = process_input(input);
    let match_spec = HashMap::from([
        (SueProperty::Children, 3),
        (SueProperty::Cats, 7),
        (SueProperty::Samoyeds, 2),
        (SueProperty::Pomeranians, 3),
        (SueProperty::Akitas, 0),
        (SueProperty::Vizslas, 0),
        (SueProperty::Goldfish, 5),
        (SueProperty::Trees, 3),
        (SueProperty::Cars, 2),
        (SueProperty::Perfumes, 1)
    ]);
    aunt_list.iter()
        .position(|test| acceptable_part_2(test, &match_spec))
        .map(|i| i as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(213));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(323));
    }
}
