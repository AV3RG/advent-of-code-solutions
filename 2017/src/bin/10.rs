use itertools::Itertools;

advent_of_code::solution!(10);

fn perform_round(list: &mut Vec<u8>, cur_pos: &mut usize, cur_skip: &mut usize, l: usize, list_len: usize) {
    if l > list_len { return; }
    let mut left = *cur_pos;
    let mut right = *cur_pos + l - 1;
    while left < right {
        list.swap(left % list_len, right % list_len);
        left += 1;
        right -= 1;
    }
    *cur_pos = (*cur_pos + l + *cur_skip) % list_len;
    *cur_skip += 1;
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut list = (0..=255).collect_vec();
    let mut cur_pos = 0;
    let mut cur_skip = 0;
    let list_len = list.len();
    input.trim().split(",").for_each(|l| {
        let l = l.trim().parse::<usize>().unwrap();
        perform_round(&mut list, &mut cur_pos, &mut cur_skip, l, list_len);
    });
    Some(list[0] as u16 * list[1] as u16)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut list = (0..=255).collect_vec();
    let mut cur_pos = 0;
    let mut cur_skip = 0;
    let list_len = list.len();
    let input = input.trim().bytes().chain(vec![17, 31, 73, 47, 23]).collect_vec();
    for _ in 0..64 {
        input.iter().for_each(|l| {
            perform_round(&mut list, &mut cur_pos, &mut cur_skip, *l as usize, list_len);
        })
    }
    let final_hash = list.chunks(16).map(|window| {
        let mut xor = window[0];
        for i in 1..window.len() {
            xor ^= window[i];
        }
        xor
    }).collect_vec();
    let result = hex::encode(final_hash);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("00423f1d90f800d9c7a913fd7cd6df24".to_string()));
    }
}
