use itertools::Itertools;

advent_of_code::solution!(5);

// TODO: Improve post LinkedList cursor implementation in rust

fn react(chain: &mut Vec<char>) -> bool {
    let option = chain.iter().tuple_windows().find_position(|(a, b)| {
        (**a as u8) + 32 == **b as u8 || (**b as u8) + 32 == **a as u8
    });
    if option.is_none() {
        return false;
    }
    let x = option.unwrap().0;
    chain.remove(x);
    chain.remove(x);
    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut vec = input.trim().chars().collect_vec();
    while react(&mut vec) {}
    Some(vec.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let vec = input.trim().chars().collect_vec();
    let max_reduction = (65..=90).map(|to_remove| {
        let mut nv = vec.clone();
        nv.retain(|c| {
            *c as u8 != to_remove && *c as u8 != (to_remove + 32)
        });
        while react(&mut nv) {}
        nv.len()
    }).min();
    max_reduction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
