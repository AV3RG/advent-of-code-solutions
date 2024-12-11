extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let trimmed_input = input.trim().to_owned();
    let mut num= 0;
    loop {
        let crypto_input = trimmed_input.clone() + &*num.to_string();
        let mut digest = Md5::new();
        digest.input(crypto_input.as_ref());
        let mut output: [u8; 16] = [0; 16];
        digest.result(&mut output);
        if output[0] == 0 && output[1] == 0 && output[2] < 16 {
            return Some(num)
        }
        num += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let trimmed_input = input.trim().to_owned();
    let mut num= 0;
    loop {
        let crypto_input = trimmed_input.clone() + &*num.to_string();
        let mut digest = Md5::new();
        digest.input(crypto_input.as_ref());
        let mut output: [u8; 16] = [0; 16];
        digest.result(&mut output);
        if output[0] == 0 && output[1] == 0 && output[2] == 0 {
            return Some(num)
        }
        num += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
