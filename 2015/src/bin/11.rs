use iter_tools::Itertools;

advent_of_code::solution!(11);

static Z_INDEX: u8 = b'z' - b'a';
static A_INDEX: u8 = 0;

fn increment_password(password: &mut [u8]) {
    let mut i = password.len() - 1;
    password[i] += 1;
    while password[i] == Z_INDEX + 1 {
        password[i] = A_INDEX;
        i -= 1;
        password[i] += 1;
    }
}

fn windows_check(password: &[u8]) -> bool {
    let filter = password.windows(2).enumerate().filter(|(_, w)| w[0] == w[1]).collect::<Vec<_>>();
    filter.len() >= 3 || (filter.len() == 2 && (filter[1].0 - filter[0].0 >= 2))
}

fn is_valid_password(password: &[u8]) -> bool {
    static INVALID_CHARS: [u8; 3] = [b'i' - b'a', b'o' - b'a', b'l' - b'a'];
    password.iter().all(|&c| !INVALID_CHARS.contains(&c)) &&
        password.windows(3).any(|w| w[2] == w[1] + 1 && w[1] == w[0] + 1) &&
        windows_check(password)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut password: [u8; 8] = [0; 8];
    input.trim().chars().enumerate().for_each(|(i, c)| {
        password[i] = (c as u8) - b'a';
    });
    increment_password(&mut password);
    let mut validity = is_valid_password(&password);
    while !validity {
        increment_password(&mut password);
        validity = is_valid_password(&password);
    }
    let collected = password.map(|c| (c + b'a') as char);
    Some(collected.iter().join("").to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    part_one(part_one(input).as_deref().unwrap()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("hjaaaabc".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("hjaaabcc".to_string()));
    }
}
