advent_of_code::solution!(9);

#[derive(Debug, Clone)]
enum Space {
    Occupied(u64, u64),
    Empty(u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sol_vec: Vec<Option<u64>> = Vec::new();
    input.trim().chars()
        .map(|c| {
            c.to_string().parse::<u64>().unwrap()
        })
        .enumerate()
        .for_each(|(index, a)| {
            if index % 2 == 0 {
                for _ in 0..a {
                    sol_vec.push(Some(index as u64 / 2))
                }

            } else {
                for _ in 0..a {
                    sol_vec.push(None)
                }
            }
        });
    let mut l = 0;
    let mut r = sol_vec.len() as i32 - 1i32;
    while l < r {
        if sol_vec.get(l as usize).unwrap().is_some() {
            l += 1;
        } else if sol_vec.get(r as usize).unwrap().is_none() {
            r -= 1;
        } else {
            sol_vec.swap(l as usize, r as usize);
            l += 1;
            r -= 1;
        }
    }
    let sol = sol_vec.iter().enumerate().map_while(|(idx, value)| {
        return if value.is_some() {
            Some(value.unwrap() * idx as u64)
        } else {
            None
        }
    }).sum();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sol_vec: Vec<Space> = input.trim().chars()
        .map(|c| {
            c.to_string().parse::<u64>().unwrap()
        })
        .enumerate()
        .map(|(index, char)| {
            return if index % 2 == 0 {
                Space::Occupied(char, index as u64 / 2)
            } else {
                Space::Empty(char)
            }
        })
        .collect();
    let mut r = sol_vec.len() as i32 - 1;
    while r > 0 {
        let mut l = 0;
        if matches!(sol_vec.get(r as usize).unwrap(), Space::Empty(_)) {
            r -= 1;
            continue;
        }
        while l < r {
            let left_space = sol_vec.get(l as usize).unwrap().clone();
            let right_space = sol_vec.get(r as usize).unwrap().clone();
            match (&left_space, &right_space) {
                (Space::Occupied(_, _), _) => {
                }
                (_, Space::Empty(_)) => {
                    panic!("Should not happen!")
                }
                (Space::Empty(empty_space), Space::Occupied(occupied_space, occupied_value)) => {
                    if empty_space == occupied_space {
                        sol_vec.swap(l as usize, r as usize);
                        break;
                    } else if *empty_space > *occupied_space {
                        sol_vec[l as usize] = Space::Occupied(*occupied_space, *occupied_value);
                        sol_vec[r as usize] = Space::Empty(*occupied_space);
                        sol_vec.insert(l as usize + 1, Space::Empty(empty_space - occupied_space));
                        break;
                    }
                }
            }
            l += 1;
        }
        r -= 1;
    };
    let mut pointer = 0i64;
    let mut sol = 0;
    for n in sol_vec.iter() {
        match n {
            Space::Occupied(space_occupied, value) => {
                sol += ((((pointer as u64 + space_occupied - 1) * (pointer as u64 + space_occupied)) / 2) - ((pointer * (pointer - 1)) / 2) as u64) * value;
                pointer += *space_occupied as i64;
            }
            Space::Empty(space_occupied) => {
                pointer += *space_occupied as i64;
            }
        }
    }
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
