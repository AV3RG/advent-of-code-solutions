use std::cmp::{max};
use std::ops::{Mul, Sub, Add};
use iter_tools::Itertools;
use regex::Regex;

advent_of_code::solution!(15);

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn score(&self) -> i64 {
        if self.capacity < 0 || self.durability < 0 || self.flavor < 0 || self.texture < 0 || self.calories < 0 {
            return 0;
        }
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Mul<i64> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Ingredient {
        Ingredient {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Add<Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(self, rhs: Ingredient) -> Ingredient {
        Ingredient {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }

}

fn process_input(input: &str) -> Vec<Ingredient> {
    let mut regex = Regex::new(r"(-?\d+)").unwrap();
    input.trim().lines().map(|line| {
        let (capacity, durability, flavor, texture, calories) = regex.find_iter(line).map(|cap| cap.as_str().parse::<i64>().unwrap()).collect_tuple().unwrap();
        Ingredient { capacity, durability, flavor, texture, calories }
    }).collect_vec()
}

pub fn part_one(input: &str) -> Option<i64> {
    let ingredients = process_input(input);
    let mut max_score = i64::MIN;
    for i in 1..=(100 - 1 - 1 - 1) {
        for j in 1..=(100 - i - 1 -1) {
            for k in 1..=(100 - i - j - 1) {
                let l = 100 - i - j - k;
                let result = (ingredients[0] * i) + (ingredients[1] * j) + (ingredients[2] * k) + (ingredients[3] * l);
                max_score = max(result.score(), max_score);
            }
        }
    }
    Some(max_score)
}

pub fn part_two(input: &str) -> Option<i64> {
    let ingredients = process_input(input);
    let mut max_score = i64::MIN;
    for i in 1..=(100 - 1 - 1 - 1) {
        for j in 1..=(100 - i - 1 -1) {
            for k in 1..=(100 - i - j - 1) {
                let l = 100 - i - j - k;
                let result = (ingredients[0] * i) + (ingredients[1] * j) + (ingredients[2] * k) + (ingredients[3] * l);
                if result.calories == 500 {
                    max_score = max(result.score(), max_score);
                }
            }
        }
    }
    Some(max_score)
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
