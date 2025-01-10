use iter_tools::Itertools;

advent_of_code::solution!(17);

fn backtracker(
    available_containers: &[i32],
    solutions: &mut Vec<Vec<i32>>,
    current_path: &mut Vec<i32>,
    target: i32,
) {
    if target == 0 {
        solutions.push(current_path.clone());
        return;
    }
    if target < 0 || available_containers.is_empty() {
        return;
    }
    let to_choose = available_containers.first().unwrap();
    let mut new_path = current_path.clone();
    new_path.push(*to_choose);
    backtracker(
        &available_containers[1..],
        solutions,
        &mut new_path,
        target - to_choose,
    );
    backtracker(
        &available_containers[1..],
        solutions,
        &mut current_path.clone(),
        target,
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let available_containers = input.trim().lines().map(|line| line.parse::<i32>().unwrap()).collect_vec();
    let mut solutions = Vec::new();
    backtracker(&available_containers, &mut solutions, &mut Vec::new(), 150);
    solutions.len().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let available_containers = input.trim().lines().map(|line| line.parse::<i32>().unwrap()).collect_vec();
    let mut solutions = Vec::new();
    backtracker(&available_containers, &mut solutions, &mut Vec::new(), 150);
    let min_len = solutions.iter().min_by_key(|s| s.len()).unwrap().len();
    solutions.iter().filter(|s| s.len() == min_len).count().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1638));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }
}
