advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.trim().parse::<usize>().unwrap();
    let mut vec = vec![0];
    let mut current_pos = 0usize;
    for i in 1..=2017 {
        let insertion_idx = ((current_pos + input) % (i)) + 1;
        if insertion_idx == i {
            vec.push(i)
        } else {
            vec.insert(insertion_idx, i);
        }
        current_pos = insertion_idx;
    }
    Some(vec[(current_pos + 1) % 2018])
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim().parse::<usize>().unwrap();
    let mut current_pos = 0usize;
    let mut value_ahead = None;
    for i in 1..=50_000_000 {
        let insertion_idx = ((current_pos + input) % (i)) + 1;
        if insertion_idx == 1 {
            value_ahead = Some(i);
        }
        current_pos = insertion_idx;
    }
    value_ahead
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(638));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1222153));
    }
}
