use iter_tools::Itertools;

advent_of_code::solution!(24);

fn backtrack(curr_path: Vec<i64>, target: i64, left_slice: &[i64], blacklisted: &Vec<i64>) -> (bool, Option<Vec<i64>>) {
    if target == 0 {
        return (true, curr_path.clone().into());
    }
    if target < 0 || left_slice.len() == 0 {
        return (false, None);
    }
    if !blacklisted.contains(&left_slice[0]) {
        let mut with_add = curr_path.clone();
        with_add.push(left_slice[0]);
        if backtrack(with_add.clone(), target - left_slice[0], &left_slice[1..], &blacklisted).0 {
            return (true, with_add.into());
        }
    }
    backtrack(curr_path.clone(), target, &left_slice[1..], &blacklisted)
}

fn find_all_of_len(nums: &Vec<i64>, len_to_find: usize, sum_needed: i64) -> Vec<Vec<i64>> {
    let mut sol = Vec::new();
    nums.iter().combinations(len_to_find).for_each(|combination| {
        let copied = combination.iter().map(|num| **num).collect_vec();
        if copied.iter().sum::<i64>() != sum_needed { return; }
        sol.push(copied);
    });
    sol
}

pub fn part_one(input: &str) -> Option<i64> {
    let packages = input.trim().lines().map(|l| {
        l.parse::<i64>().unwrap()
    }).sorted().collect_vec();
    let weight_per_group = packages.iter().sum::<i64>() / 3;


    for i in 1..packages.len() {
        let solutions = find_all_of_len(&packages, i, weight_per_group);
        if solutions.len() == 0 {
            continue;
        }
        let sol = solutions.iter().sorted_by_key(|poss| {
            poss.iter().fold(1, |acc, pos| { acc * pos })
        }).find(|poss| {
            backtrack(Vec::new(), weight_per_group, &packages, poss).0
        }).unwrap();
        return Some(sol.iter().fold(1, |acc, pos| { acc * pos }));
    }
    None
}

pub fn part_two(input: &str) -> Option<i64> {
    let packages = input.trim().lines().map(|l| {
        l.parse::<i64>().unwrap()
    }).sorted().collect_vec();
    let weight_per_group = packages.iter().sum::<i64>() / 4;
    for i in 1..packages.len() {
        let solutions = find_all_of_len(&packages, i, weight_per_group);
        if solutions.len() == 0 {
            continue;
        }
        let sol = solutions.iter().sorted_by_key(|poss| {
            poss.iter().fold(1, |acc, pos| { acc * pos })
        }).find(|poss| {
            let sat1 = backtrack(Vec::new(), weight_per_group, &packages, poss);
            if !sat1.0 { return false; }
            let mut p2 = poss.to_owned().to_owned();
            p2.extend(sat1.1.unwrap().into_iter());
            let sat2 = backtrack(Vec::new(), weight_per_group, &packages, &p2);
            sat2.0
        });
        if sol.is_none() { continue; }
        let sol = sol.unwrap();
        return Some(sol.iter().fold(1, |acc, pos| { acc * pos }));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(99));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(44));
    }
}
