advent_of_code::solution!(13);

struct FirewallLevel {
    max_len: u32
}

impl FirewallLevel {
    fn is_at_0(&self, time: u32) -> bool {
        time % ((self.max_len - 1) * 2) == 0
    }
}

fn process_input(input: &str) -> Vec<FirewallLevel> {
    let mut vec = Vec::new();
    input.trim().lines().for_each(|line| {
        let (level, max_len) = line.split_once(": ").unwrap();
        let level = level.parse::<usize>().unwrap();
        let firewall = FirewallLevel {
            max_len: max_len.parse::<u32>().unwrap()
        };
        while level > vec.len() {
            vec.push(FirewallLevel { max_len: u16::MAX as u32 });
        }
        vec.push(firewall);
    });
    vec
}


pub fn part_one(input: &str) -> Option<u32> {
    let vec = process_input(input);
    let mut total = 0;
    for i in 0..vec.len() {
        if vec[i].is_at_0(i as u32) {
            total += (i as u32) * vec[i].max_len
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut delay = 0;
    let vec = process_input(input);
    loop {
        let mut sol = true;
        for i in 0..vec.len() {
            if vec[i].is_at_0(i as u32 + delay) {
                delay += 1;
                sol = false;
                break;
            }
        }
        if sol {
            return Some(delay)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
