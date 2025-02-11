use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(15);

fn advance_machine(machine: &mut Vec<(u16, u16)>) {
    for i in 0..machine.len() {
        machine[i].1 = (machine[i].1 + 1) % machine[i].0
    }
}

fn process_input(input: &str) -> Vec<(u16, u16)> {
    let regex = Regex::new(r"(\d+)").unwrap();
    input.trim().lines().map(|l| {
        let mut captured = regex.find_iter(l).collect_vec();
        (
            captured[1].as_str().parse::<u16>().unwrap(),
            captured[3].as_str().parse::<u16>().unwrap()
        )
    }).collect_vec()
}

fn do_time_adjustment(machine: &mut Vec<(u16, u16)>) {
    for i in 0..machine.len() {
        machine[i].1 = (machine[i].1 + (i as u16)) % machine[i].0
    }
    advance_machine(machine);
}

fn find_delay(machine: &mut Vec<(u16, u16)>) -> u64 {
    let mut delay = 0;
    while !machine.iter().all(|a| a.1 == 0) {
        advance_machine(machine);
        delay += 1;
    }
    delay
}

//TODO optimise with LCM
pub fn part_one(input: &str) -> Option<u64> {
    let mut machine = process_input(input);
    do_time_adjustment(&mut machine);
    Some(find_delay(&mut machine))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut machine = process_input(input);
    machine.push((11, 0));
    do_time_adjustment(&mut machine);
    Some(find_delay(&mut machine))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
