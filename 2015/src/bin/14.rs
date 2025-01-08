use iter_tools::Itertools;
use regex::Regex;

advent_of_code::solution!(14);

fn process_input(input: &str) -> Vec<(i32, i32, i32)> {
    let regex = Regex::new(r"(?P<number>\d+)").unwrap();
    input.trim().lines().map(|line| {
        let (speed, time_maintain, time_rest) = regex.find_iter(line).map(|m| m.as_str().parse::<i32>().unwrap()).collect_tuple().unwrap();
        (speed, time_maintain, time_rest)
    }).collect_vec()
}

fn simulate_reindeer((speed, time_maintain, time_rest): &(i32, i32, i32), mut simulate_seconds: i32) -> i32 {
    let mut distance = 0;
    while simulate_seconds > 0 {
        if time_maintain <= &simulate_seconds {
            distance += speed * time_maintain;
            simulate_seconds -= (time_maintain + time_rest) as i32;
        } else {
            distance += speed * simulate_seconds;
            simulate_seconds = 0;
        }
    }
    distance
}

fn complete_position_by_time((speed, time_maintain, time_rest): &(i32, i32, i32), simulate_seconds: i32) -> Vec<i32> {
    let mut positions = vec![0; simulate_seconds as usize];
    let mut distance = 0;
    let mut rest_pending = 0;
    let mut run_pending = *time_maintain;
    let mut current_time = 0;
    while (current_time as i32) < simulate_seconds {
        if rest_pending > 0 {
            rest_pending -= 1;
            positions[current_time] = distance;
            current_time += 1;
            if rest_pending == 0 {
                run_pending = *time_maintain;
                continue;
            }
        } else if run_pending > 0 {
            run_pending -= 1;
            distance += speed;
            positions[current_time] = distance;
            current_time += 1;
            if run_pending == 0 {
                rest_pending = *time_rest;
                continue;
            }
        } else {
            panic!("Neither running nor resting");
        }
    }
    positions
}

pub fn part_one(input: &str) -> Option<i32> {
    let reindeer_list = process_input(input);
    reindeer_list.iter().map(|reindeer| simulate_reindeer(reindeer, 2503)).max()
}

pub fn part_two(input: &str) -> Option<i32> {
    let reindeer_list = process_input(input);
    let mapping = reindeer_list.iter().map(|reindeer| complete_position_by_time(reindeer, 2503)).collect_vec();
    let mut points = vec![0; reindeer_list.len()];
    for i in 0..2503 {
        let mut max_position = -1;
        let mut max_indices = Vec::new();
        for reindeer in 0..reindeer_list.len() {
            if mapping[reindeer][i] > max_position {
                max_position = mapping[reindeer][i];
                max_indices = vec![reindeer];
            } else if mapping[reindeer][i] == max_position {
                max_indices.push(reindeer);
            }
        }
        for reindeer in max_indices {
            points[reindeer] += 1;
        }
    }
    points.iter().max().map(|&p| p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2660));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1564));
    }
}
