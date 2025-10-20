use itertools::Itertools;
use pathfinding::prelude::astar;
use smallvec::SmallVec;
use std::collections::{HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

advent_of_code::solution!(11);

const FLOORS: usize = 4;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Item {
    Generator(u8),
    Microchip(u8),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Floor {
    items: SmallVec<[Item; 8]>,
}

#[derive(Clone, Debug)]
struct Building {
    floors: [Floor; FLOORS],
    elevator: usize,
    num_elements: u8,
}

impl Floor {
    fn is_safe(&self) -> bool {
        // A floor is safe if no microchip is powered by another generator
        if self.items.iter().all(|x| matches!(x, Item::Microchip(_))) {
            return true;
        }
        let gens: HashSet<_> = self
            .items
            .iter()
            .filter_map(|i| match i {
                Item::Generator(id) => Some(*id),
                _ => None,
            })
            .collect();

        for item in &self.items {
            if let Item::Microchip(id) = item {
                if !gens.contains(id) && !gens.is_empty() {
                    return false;
                }
            }
        }
        true
    }
}

impl Building {
    fn is_goal(&self) -> bool {
        self.floors[0..FLOORS - 1]
            .iter()
            .all(|f| f.items.is_empty())
    }

    // Canonical representation for hashing & equality
    fn canonical_signature(&self) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for id in 0..self.num_elements {
            let gf = self
                .floors
                .iter()
                .position(|f| f.items.iter().any(|x| *x == Item::Generator(id)))
                .unwrap();
            let cf = self
                .floors
                .iter()
                .position(|f| f.items.iter().any(|x| *x == Item::Microchip(id)))
                .unwrap();
            pairs.push((gf, cf));
        }
        pairs.sort();
        pairs
    }

    fn canonical_key(&self) -> (usize, Vec<(usize, usize)>) {
        (self.elevator, self.canonical_signature())
    }

    fn heuristic(&self) -> usize {
        // Sum of item floor positions weighted by distance from top
        self.floors
            .iter()
            .enumerate()
            .map(|(i, f)| f.items.len() * (FLOORS - i - 1))
            .sum()
    }

    fn next_states(&self) -> Vec<Building> {
        let mut next = Vec::new();
        let current_items = &self.floors[self.elevator].items;
        if current_items.is_empty() {
            return next;
        }

        let directions: SmallVec<[isize; 2]> = match self.elevator {
            0 => smallvec::smallvec![1],
            3 => smallvec::smallvec![-1],
            _ => smallvec::smallvec![-1, 1],
        };

        for dir in directions {
            let new_floor = (self.elevator as isize + dir) as usize;
            if dir < 0
                && self.floors[..self.elevator]
                .iter()
                .all(|f| f.items.is_empty())
            {
                continue; // Don't go down if nothing below
            }

            // Generate item combinations: 2 first, then 1
            for combo in current_items
                .iter()
                .combinations(2)
                .chain(current_items.iter().combinations(1))
            {
                let mut clone = self.clone();
                let from_floor = &mut clone.floors[self.elevator].items;
                for to_move in &combo {
                    from_floor.retain(|x| x != *to_move);
                }
                let dest = &mut clone.floors[new_floor].items;
                dest.extend(combo.iter().cloned().cloned());
                clone.elevator = new_floor;

                if clone.floors[self.elevator].is_safe()
                    && clone.floors[new_floor].is_safe()
                {
                    next.push(clone);
                }
            }
        }
        next
    }
}

impl FromStr for Building {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut elemental_mapping = Vec::new();
        let mut floors_arr: [Floor; FLOORS] =
            std::array::from_fn(|_| Floor { items: SmallVec::new() });

        for (i, line) in input.lines().enumerate() {
            let line = line.to_lowercase().replace(".", "").replace("and", ",");
            for section in line.split(',') {
                let words = section.trim().split_whitespace().collect_vec();
                if words.len() < 2 {
                    continue;
                }
                if let Some(kind) = words.last() {
                    if *kind == "generator" {
                        let element = words[words.len() - 2];
                        let id = register_element(element, &mut elemental_mapping);
                        floors_arr[i].items.push(Item::Generator(id));
                    } else if *kind == "microchip" {
                        let element =
                            words[words.len() - 2].replace("-compatible", "");
                        let id = register_element(&element, &mut elemental_mapping);
                        floors_arr[i].items.push(Item::Microchip(id));
                    }
                }
            }
        }

        Ok(Self {
            floors: floors_arr,
            elevator: 0,
            num_elements: elemental_mapping.len() as u8,
        })
    }
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.canonical_key() == other.canonical_key()
    }
}
impl Eq for Building {}
impl Hash for Building {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical_key().hash(state);
    }
}

fn register_element(
    name: &str,
    existing: &mut Vec<String>,
) -> u8 {
    if let Some(idx) = existing.iter().position(|x| x == name) {
        idx as u8
    } else {
        existing.push(name.to_string());
        (existing.len() - 1) as u8
    }
}

fn solve_a_star(initial: Building) -> Option<usize> {
    let result = astar(
        &initial,
        |b| b.next_states().into_iter().map(|s| (s, 1)),
        |b| b.heuristic(),
        |b| b.is_goal(),
    );
    result.map(|(_, cost)| cost)
}

pub fn part_one(input: &str) -> Option<usize> {
    let building: Building = input.parse().unwrap();
    solve_a_star(building)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut building: Building = input.parse().unwrap();
    building.floors[0].items.push(Item::Microchip(5));
    building.floors[0].items.push(Item::Microchip(6));
    building.floors[0].items.push(Item::Generator(5));
    building.floors[0].items.push(Item::Generator(6));
    building.num_elements += 2;
    solve_a_star(building)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
