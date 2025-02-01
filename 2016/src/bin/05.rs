use aoc_utils::hex_utils::hex_index_to_char;
use crypto::digest::Digest;
use crypto::md5::Md5;
use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let input = input.trim();
    let mut md5 = Md5::new();
    let mut num = 0;
    let mut output = Vec::new();
    while output.len() < 8 {
        let num_string = num.to_string();
        md5.input_str(&*(input.to_string() + &num_string));
        let mut result = [0u8; 16];
        md5.result(&mut result);
        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            let hex = result[2] % 16;
            let charred = hex_index_to_char(hex);
            output.push(charred as char);
        }
        num += 1;
        md5.reset();
    };
    output.iter().join("").into()
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.trim();
    let mut md5 = Md5::new();
    let mut num = 0;
    let mut output = vec![None; 8];
    let mut counter = 8;
    while counter > 0 {
        let num_string = num.to_string();
        md5.input_str(&*(input.to_string() + &num_string));
        let mut result = [0u8; 16];
        md5.result(&mut result);
        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            let hex = result[2] % 16;
            if hex >= 8 {
                num += 1;
                md5.reset();
                continue;
            }
            if output[hex as usize].is_none() {
                output[hex as usize] = Some(hex_index_to_char(result[3] / 16));
                counter -= 1;
            }
        }
        num += 1;
        md5.reset();
    };
    output.into_iter().map(|x| x.unwrap()).collect::<String>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".to_string()));
    }
}
