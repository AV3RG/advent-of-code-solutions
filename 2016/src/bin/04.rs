use std::collections::HashMap;
use itertools::Itertools;

advent_of_code::solution!(4);

fn apply_caesar(strings: &Vec<&str>, shift: u8) -> Vec<String> {
    strings.iter().map(|s| {
        s.chars().map(|c| ((((c as u8) - ('a' as u8) + shift) % 26) + 'a' as u8) as char).collect::<String>()
    }).collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    input.trim().lines().map(|line| {
        let vec = line.split("-").into_iter().collect::<Vec<&str>>();
        let mut counter = HashMap::new();
        vec.iter().rev().skip(1).for_each(|sub| {
            sub.chars().for_each(|c| {
                *counter.entry(c).or_insert(0) += 1;
            })
        });
        let checksum = vec.last().unwrap();
        let chars = &checksum[(checksum.len() - 6)..(checksum.len() - 1)];
        let iter = counter.iter().sorted_by(|entry1, entry2| {
            if entry1.1 == entry2.1 {
                entry1.0.cmp(&entry2.0)
            } else {
                entry2.1.cmp(&entry1.1)
            }
        }).map(|entry| entry.0).collect::<Vec<_>>();
        return if iter[0..5].iter().join("") == chars {
            checksum[0..checksum.len() - 7].parse::<u64>().unwrap()
        } else {
            0
        }
    }).sum::<u64>().into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input.trim().lines().find_map(|line| {
        let mut split = line.split("-").into_iter().collect::<Vec<&str>>();
        let checksum = split.last().unwrap();
        let total_shift = checksum[0..checksum.len() - 7].parse::<u64>().unwrap();
        let shift = (total_shift % 26) as u8;
        split.remove(split.len() - 1);
        if apply_caesar(&split, shift).contains(&"northpole".to_string()) {
            return total_shift.into()
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
