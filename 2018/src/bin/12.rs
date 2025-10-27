use std::collections::HashMap;

advent_of_code::solution!(12);

fn process_input(input: &str, side_len: usize) -> (Vec<u16>, HashMap<u16, u16>) {
    let mut state = vec![0u16; side_len * 2];
    let (initial_state, rules) = input.trim().split_once("\n\n").unwrap();
    initial_state.split_once(": ").unwrap().1.chars().enumerate().for_each(|(i, c)| {
        if c == '#' {
            state[i + side_len] = 1;
        }
    });
    let mut rule_map = HashMap::new();
    rules.trim().lines().for_each(|rule| {
        let (lhs, rhs) = rule.split_once(" => ").unwrap();
        let mut rule_id = 0u16;
        lhs.chars().for_each(|c| {
            rule_id <<= 1;
            rule_id |= if c == '#' { 1 } else { 0 };
        });
        rule_map.insert(rule_id, if rhs == "." { 0 } else { 1 });
    });
    (state, rule_map)
}

fn tick(state: &mut Vec<u16>, rule_map: &HashMap<u16, u16>) {
    let l = state.len();
    state[0] = (*rule_map.get(&(state[0] << 2 | state[1] << 1 | state[2] << 0)).unwrap_or(&0)) << 8;
    for i in 2..l - 1 - 2 {
        let mat = state[i - 2] << 4 | state[i - 1] << 3 | state[i] << 2 | state[i + 1] << 1 | state[i + 2] << 0;
        let mat = mat & 0b11111111;
        let x = rule_map.get(&mat).unwrap_or(&0);
        state[i] |= (*x) << 8
    }
    state[l - 1] = (*rule_map.get(&(state[l - 1] << 2 | state[l - 2] << 3 | state[l - 3] << 4)).unwrap_or(&0)) << 8;
    state.iter_mut().for_each(|x| *x >>= 8);
}

fn count_pots(state: &Vec<u16>, side_len: i32) -> i32 {
    state.iter().enumerate().map(|(i, x)| if *x == 0 { 0 } else { i as i32 - side_len } ).sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    let side_len = 1000i32;
    let (mut state, rules) = process_input(input, side_len as usize);
    for _ in 0..20 {
        tick(&mut state, &rules);
    }
    Some(count_pots(&state, side_len))
}

pub fn part_two(input: &str) -> Option<i128> {
    let side_len = 500i32;
    let until_stabilise = 300;
    let (mut state, rules) = process_input(input, side_len as usize);
    for _ in 0..until_stabilise {
        tick(&mut state, &rules);
    }
    let till_now = count_pots(&state, side_len);
    tick(&mut state, &rules);
    let difference = count_pots(&state, side_len) - till_now;
    Some(((50_000_000_000 - until_stabilise) * difference as i128) + till_now as i128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(325));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
