advent_of_code::solution!(10);

fn simulate(input: &str, iterations: u8) -> String {
    let mut owned_input = input.to_string();
    for _ in 0..iterations {
        let mut last_char = owned_input.chars().next().unwrap();
        let mut last_char_count = 1;
        let mut final_string = String::new();
        owned_input.trim().chars().skip(1).for_each(|c| {
            if c == last_char {
                last_char_count += 1;
            } else {
                final_string.push_str(&last_char_count.to_string());
                final_string.push(last_char);
                last_char = c;
                last_char_count = 1;
            }
        });
        final_string.push_str(&last_char_count.to_string());
        final_string.push(last_char);
        owned_input = final_string.to_owned();
    }
    owned_input
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(simulate(input, 40).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(simulate(input, 50).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(237746));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3369156));
    }
}
