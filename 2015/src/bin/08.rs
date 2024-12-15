use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let hex_regex = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
    let sol = input.trim().lines().map(|line| {
        let line_without_apos = line.trim().trim_matches('"');
        let non_hex_escaped = line_without_apos.replace("\\\"", "\"").replace("\\\\", "\\");
        let r#final = hex_regex.replace_all(&non_hex_escaped, "X").to_string();
        (line.len() - r#final.len()) as u32
    }).sum::<u32>();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
        input.trim().lines().map(|line| {
        let mod_len = line.replace("\\", "\\\\").replace("\"", "\\\"").len();
        (mod_len - line.len() + 2) as u32
    }).sum::<u32>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
