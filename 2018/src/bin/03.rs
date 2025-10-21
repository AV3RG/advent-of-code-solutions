use std::str::FromStr;
use itertools::Itertools;

advent_of_code::solution!(3);

struct Claim {
    num: usize,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

fn intersect(start1: usize, end1: usize, start2: usize, end2: usize) -> bool {
    !(start1 > end2 || start2 > end1)
}

impl FromStr for Claim {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, useful) = s.split_once(" @ ").unwrap();
        let (start, size) = useful.split_once(": ").unwrap();
        let (start_x, start_y) = start.trim().split_once(",").unwrap();
        let (start_x, start_y) = (start_x.parse::<usize>().unwrap(), start_y.parse::<usize>().unwrap());
        let (size_x, size_y) = size.trim().split_once("x").unwrap();
        let (size_x, size_y) = (size_x.parse::<usize>().unwrap(), size_y.parse::<usize>().unwrap());
        let num = num.trim().replace("#", "").parse::<usize>().unwrap();
        Ok(Claim {
            num,
            start_x,
            start_y,
            end_x: start_x + size_x,
            end_y: start_y + size_y,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = [[0u8; 1000]; 1000];
    let mut count = 0;
    input.trim().lines().for_each(|line| {
        let claim = Claim::from_str(&line).unwrap();
        for x in claim.start_x..claim.end_x {
            for y in claim.start_y..claim.end_y {
                if grid[x][y] == 0 {
                    grid[x][y] = 1;
                } else if grid[x][y] == 1 {
                    grid[x][y] = 2;
                    count += 1;
                }
            }
        }
    });
    Some(count)

}

pub fn part_two(input: &str) -> Option<u64> {
    let vec = input.trim().lines().map(|line| Claim::from_str(line).unwrap()).collect_vec();
    for claim in vec.iter() {
        let mut satisfied = true;
        for other in vec.iter() {
            if claim.num == other.num { continue; }
            if intersect(claim.start_x, claim.end_x, other.start_x, other.end_x) && intersect(claim.start_y, claim.end_y, other.start_y, other.end_y) {
                satisfied = false;
                break;
            }
        }
        if satisfied {
            return Some(claim.num as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
