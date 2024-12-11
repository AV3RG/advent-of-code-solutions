use std::collections::{HashSet};
use std::ops::Not;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let vowels_arr = ['a', 'e', 'i', 'o', 'u'];
    let forbidden = ["ab", "cd", "pq", "xy"];
    let sol = input.trim().lines().filter(|line| {
        let mut vowels = 0u8;
        let mut repeat = false;
        let l = line.len();
        let line_indices = line.chars().collect::<Vec<_>>();

        for i in 0..l - 1 {
            if vowels_arr.contains(&line_indices[i]) {
                vowels += 1
            }
            if !repeat && line_indices[i] == line_indices[i + 1] {
                repeat = true
            }
            let a = line_indices[i].to_string() + &*line_indices[i + 1].to_string();
            if forbidden.contains(&&*a) {
                repeat = false;
                break;
            }
        }
        if vowels_arr.contains(&line_indices.last().unwrap()) {
            vowels += 1;
        }
        vowels >= 3 && repeat
    }).count();
    Some(sol as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trimmed_input = input.trim();
    let sol = trimmed_input.lines().filter(|line| {
        let chars = line.chars().collect::<Vec<_>>();
        let mut found_windows = HashSet::new();
        let mut last_window = ['1', '1'];

        let window = chars.windows(2).any(|window| {
            return if found_windows.contains(window) && last_window != window {
                true
            } else {
                found_windows.insert(window);
                last_window = <[char; 2]>::try_from(window).unwrap();
                false
            }
        });
        if window.not() {
            return false
        }
        for i in 1..chars.len() - 1 {
            if chars[i - 1] == chars[i +  1] {
                return true
            }
        }
        return false

    }).count();
    Some(sol as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}
