use std::collections::HashSet;
use std::ops::{Add, Div, Mul};
use std::sync::LazyLock;
use itertools::Itertools;
use regex::Regex;
use aoc_utils::quad_utils::find_intersection;

advent_of_code::solution!(20);

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct XYZ {
    x: i64,
    y: i64,
    z: i64
}

impl XYZ {

    fn calculate_absolute(&self) -> f64 {
        ((self.x.abs().pow(2) + self.y.abs().pow(2) + self.z.abs().pow(2)) as f64).powf(0.5)
    }

}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
struct Particle  {
    num: u16,
    position: XYZ,
    velocity: XYZ,
    acceleration: XYZ,
    removed: bool
}

impl Particle {

    fn position_at_time(&self, time: i64) -> XYZ {
        let time = time as f64;
        let (x, y, z) = (
            (self.acceleration.x as f64).div(2.0).mul(time).mul(time) + (self.velocity.x as f64).add((self.acceleration.x as f64).div(2.0)).mul(time) + self.position.x as f64,
            (self.acceleration.y as f64).div(2.0).mul(time).mul(time) + (self.velocity.y as f64).add((self.acceleration.y as f64).div(2.0)).mul(time) + self.position.y as f64,
            (self.acceleration.z as f64).div(2.0).mul(time).mul(time) + (self.velocity.z as f64).add((self.acceleration.z as f64).div(2.0)).mul(time) + self.position.z as f64,
        );
        XYZ { x: x as i64, y: y as i64, z: z as i64 }
    }

    fn find_collision(&self, other: &Particle) -> Option<i64> {
        let roots = find_intersection(
            (self.acceleration.x as f32) / 2.0,
            (self.velocity.x as f32) + ((self.acceleration.x as f32) / 2.0),
            self.position.x as f32,
            (other.acceleration.x as f32) / 2.0,
            other.velocity.x as f32 + ((other.acceleration.x as f32) / 2.0),
            other.position.x as f32
        );
        if roots.is_none() {
            return None;
        }
        let (r1, r2) = roots.unwrap();
        if !r1.is_sign_negative() && ((r1 as i64) as f64) == r1 && self.position_at_time(r1 as i64) == other.position_at_time(r1 as i64) {
            return Some(r1 as i64);
        }
        if !r2.is_sign_negative() && ((r2 as i64) as f64) == r2 && self.position_at_time(r2 as i64) == other.position_at_time(r2 as i64) {
            return Some(r2 as i64);
        }
        None
    }

    fn serialize(s: &str, num: u16) -> Self {
        static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"p=<(?P<px>\s?-?\d+),(?P<py>\s?-?\d+),(?P<pz>\s?-?\d+)>, v=<(?P<vx>\s?-?\d+),(?P<vy>\s?-?\d+),(?P<vz>\s?-?\d+)>, a=<(?P<ax>\s?-?\d+),(?P<ay>\s?-?\d+),(?P<az>\s?-?\d+)>").unwrap());
        let captures = REGEX.captures(s).unwrap();
        Particle {
            num,
            position: XYZ {
                y: captures["py"].trim().parse().unwrap(),
                x: captures["px"].trim().parse().unwrap(),
                z: captures["pz"].trim().parse().unwrap()
            },
            velocity: XYZ {
                x: captures["vx"].trim().parse().unwrap(),
                y: captures["vy"].trim().parse().unwrap(),
                z: captures["vz"].trim().parse().unwrap()
            },
            acceleration: XYZ {
                x: captures["ax"].trim().parse().unwrap(),
                y: captures["ay"].trim().parse().unwrap(),
                z: captures["az"].trim().parse().unwrap()
            },
            removed: false
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let vec = input.trim().lines().enumerate().map(|l| {
        Particle::serialize(l.1, l.0 as u16)
    }).collect_vec();
    Some(vec.iter().sorted_by(|p1, p2|
        p2.acceleration.calculate_absolute().total_cmp(&p1.acceleration.calculate_absolute()).then(
            p2.velocity.calculate_absolute().total_cmp(&p1.velocity.calculate_absolute()).then(
                p2.position.calculate_absolute().total_cmp(&p1.position.calculate_absolute())
            )
        )
    ).last().unwrap().num)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = input.trim().lines().enumerate().map(|l| {
        Particle::serialize(l.1, l.0 as u16)
    }).collect::<HashSet<Particle>>();
    let mut collisions = Vec::new();
    vec.iter().tuple_combinations().for_each(|(a, b)| {
        let possible = a.find_collision(b);
        if possible.is_none() {
            return;
        }
        collisions.push((possible.unwrap(), *a, *b));
    });
    let map = collisions.iter().into_group_map_by(|a| a.0);
    map.keys().sorted().for_each(|time| {
        map.get(time).unwrap().iter().for_each(|collision| {
            vec.remove(&collision.1);
            vec.remove(&collision.2);
        })
    });
    return Some(vec.len());
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
        assert_eq!(result, Some(1));
    }
}
