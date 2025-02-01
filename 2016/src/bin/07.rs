use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(7);

fn supports_tls(part: &str) -> bool {
    part.chars().tuple_windows().any(|(a, b, c, d)| {
        a == d && b == c && a != b
    })
}

fn ssl_finder(line: &str, regex: &Regex) -> Vec<(char, char, char)> {
    regex.find_iter(line).map(|m| m.as_str()).map(|group| {
        group.chars().tuple_windows().filter_map(|(a, b, c)| {
            if a == c && a != b {
                (a, b, c).into()
            } else {
                None
            }
        })
    }).flatten().collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let regex1 = Regex::new(r"((\w+)(\[|$))").unwrap();
    let regex2 = Regex::new(r"\[\w+]").unwrap();
    input.trim().lines().filter(|l| {
        regex1.find_iter(l).any(|finding| supports_tls(finding.as_str())) &&
            regex2.find_iter(l).all(|finding| !supports_tls(finding.as_str()))
    }).count().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let regex1 = Regex::new(r"((\w+)(\[|$))").unwrap();
    let regex2 = Regex::new(r"(\[\w+])").unwrap();
    input.trim().lines().filter(|l| {
        let vec1 = ssl_finder(l, &regex1);
        let vec2 = ssl_finder(l, &regex2);
        let sol = vec1.iter().any(|e1| {
            vec2.iter().any(|e2| {
                e1.0 == e2.1 && e1.1 == e2.0
            })
        });
        sol
    }).count().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, 2.into());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, 3.into());
    }
}
