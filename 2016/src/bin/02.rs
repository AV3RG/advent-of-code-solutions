use aoc_utils::tuple_maths::tuple_add;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut line_rep = 5;
    input.trim().lines().map(|line| {
        line.trim().chars().for_each(|c| {
            match c {
                'U' => if line_rep > 3 { line_rep -= 3 },
                'D' => if line_rep < 7 { line_rep += 3 },
                'L' => if line_rep % 3 != 1 { line_rep -= 1 },
                'R' => if line_rep % 3 != 0 { line_rep += 1 },
                _  => panic!("Invalid character in input"),
            }
        });
        line_rep
    }).fold(0, |acc, x| {
        acc * 10 + x
    }).into()
}

pub fn part_two(input: &str) -> Option<String> {
    let pos_wise_nums = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '2', '3', '4', '0'],
        vec!['5', '6', '7', '8', '9'],
        vec!['0', 'A', 'B', 'C', '0'],
        vec!['0', '0', 'D', '0', '0'],
    ];
    let mut pos = (0i8, 2i8);
    let range = 0..5;
    input.trim().lines().map(|line| {
        line.trim().chars().for_each(|c| {
            let v = match c {
                'U' => (0, -1),
                'D' => (0, 1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _  => panic!("Invalid character in input"),
            };
            let new_pos = tuple_add(v, pos);
            if !range.contains(&new_pos.0) || !range.contains(&new_pos.1) {
                return;
            }
            if pos_wise_nums[new_pos.1 as usize][new_pos.0 as usize] != '0' {
                pos = new_pos;
            }
        });
        pos_wise_nums[pos.1 as usize][pos.0 as usize]
    }).collect::<String>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1985));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, "5DB3".to_string().into());
    }
}
