advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let sol = input.trim().lines()
        .map(|l| {
            return l.split_whitespace().map(|part| {
                return part.parse::<i16>().unwrap()
            }).collect::<Vec<i16>>()
        })
        .filter(|part| {
            if part.len() == 1 {
                return true;
            }
            if part[0] < part[1] {
                return part.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
            } else {
                return part.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
            }
        })
        .count() as u32;
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sol = input.trim().lines()
        .map(|l| {
            return l.split_whitespace().map(|part| {
                return part.parse::<i16>().unwrap()
            }).collect::<Vec<i16>>()
        })
        .filter(|part| {
            if part.len() == 1 {
                return true;
            }
            for i in 0..part.len() {
                let mut to_use_part = part.clone();
                to_use_part.remove(i);
                if to_use_part.len() == 1 {
                    return true;
                }
                if to_use_part[0] < to_use_part[1] {
                    if to_use_part.windows(2).all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3) {
                        return true;
                    }
                }
                else {
                    if to_use_part.windows(2).all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3) {
                        return true;
                    }
                }
            }
            return false;
        })
        .count() as u32;
    Some(sol)
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
        assert_eq!(result, Some(4));
    }
}
