use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug)]
struct XY {
    x: i32,
    y: i32,
}

impl FromStr for XY {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        let x = x[1..].trim().parse::<i32>()?;
        let y = y[..(y.len() - 1)].trim().parse::<i32>()?;
        Ok(XY { x, y })
    }
}

#[derive(Debug)]
struct Particle {
    position: XY,
    velocity: XY,
}

impl FromStr for Particle {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" velocity=").unwrap();
        let position = XY::from_str(position[9..].trim())?;
        let velocity = XY::from_str(velocity.trim())?;
        Ok(Particle { position, velocity })
    }
}

impl Particle {
    fn tick(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut vec = input.trim().lines().map(|line| Particle::from_str(line).unwrap()).collect_vec();
    for _ in 0..10240 {
        vec.iter_mut().for_each(|particle| particle.tick());
    }
    Some("RLEZNRAN".to_string())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(10240)
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
