use std::collections::HashMap;

advent_of_code::solution!(19);

fn process_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (input_keys, to_make_str) = input.trim().split_once("\n\n").unwrap();
    let mut keys = Vec::new();
    let to_make = to_make_str.lines().collect::<Vec<_>>();
    input_keys.split(", ").for_each(|k| {
        keys.push(k);
    });
    (to_make, keys)
}


fn check_possible(test: &str, keys: &Vec<&str>, cache: &mut HashMap<String, bool>) -> bool {
    if test.len() == 0 {
        return true
    }
    if cache.contains_key(&test.to_string()) {
        return *cache.get(&test.to_string()).unwrap()
    }
    for ele in keys {
        if ele.len() > test.len() {
            continue;
        }
        if test[..ele.len()] != **ele {
            continue;
        }
        if check_possible(&test[ele.len()..], keys, cache) {
            cache.insert(test[ele.len()..].to_string(), true);
            return true
        }
    }
    cache.insert(test.to_string(), false);
    false
}

fn ways_to_make(test: &str, keys: &Vec<&str>, cache: &mut HashMap<String, u64>) -> u64 {
    if test.len() == 0 {
        return 1
    }
    if cache.contains_key(&test.to_string()) {
        return *cache.get(&test.to_string()).unwrap()
    }
    let ways = keys.iter().map(|ele| {
        if ele.len() > test.len() {
            return 0u64;
        }
        if test[..ele.len()] != **ele {
            return 0;
        }
        return ways_to_make(&test[ele.len()..], keys, cache);
    }).sum();
    cache.insert(test.to_string(), ways);
    return ways;
}

pub fn part_one(input: &str) -> Option<u64> {
    let (to_make, keys) = process_input(input);
    let mut cache = HashMap::new();
    let sol = to_make.iter().filter(|test| {
        check_possible(test, &keys, &mut cache)
    }).count();
    Some(sol as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (to_make, keys) = process_input(input);
    let mut cache = HashMap::new();
    let sol = to_make.iter().map(|test| {
        ways_to_make(test, &keys, &mut cache)
    }).sum();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
