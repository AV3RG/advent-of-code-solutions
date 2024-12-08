use aoc_utils::combinatorics::pair_combinations;
use aoc_utils::grid_utils::{count_in_grid, grid_item_positions};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let horizontal_range = 0..m as i32;
    let vertical_range = 0..n as i32;
    let g_clone_for_pos = grid.clone();
    let nodes = grid_item_positions(&g_clone_for_pos);
    nodes.iter().for_each(|(node, positions)| {
        if **node == '.' {
            return;
        }
        let combinations = pair_combinations(positions);
        combinations.iter().for_each(|(pos1, pos2)| {
            let (y1, x1) = *pos1;
            let (y2, x2) = *pos2;

            let int_safe_y1 = (*y1) as i32;
            let int_safe_x1 = (*x1) as i32;
            let int_safe_y2 = (*y2) as i32;
            let int_safe_x2 = (*x2) as i32;

            let (py1, px1) = (int_safe_y1 + (int_safe_y1 - int_safe_y2), int_safe_x1 + (int_safe_x1 - int_safe_x2));
            let (py2, px2) = (int_safe_y2 + (int_safe_y2 - int_safe_y1), int_safe_x2 + (int_safe_x2 - int_safe_x1));

            if horizontal_range.contains(&px1) && vertical_range.contains(&py1) {
                grid[py1 as usize][px1 as usize] = '#';
            }
            if horizontal_range.contains(&px2) && vertical_range.contains(&py2) {
                grid[py2 as usize][px2 as usize] = '#';
            }
        });
    });
    Some(count_in_grid(&grid, &'#'))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let (n, m) = (grid.len(), grid[0].len());
    let horizontal_range = 0..m as i32;
    let vertical_range = 0..n as i32;
    let g_clone_for_pos = grid.clone();
    let nodes = grid_item_positions(&g_clone_for_pos);
    nodes.iter().for_each(|(node, positions)| {
        if **node == '.' {
            return;
        }
        let combinations = pair_combinations(positions);
        combinations.iter().for_each(|(pos1, pos2)| {
            let (y1, x1) = *pos1;
            let (y2, x2) = *pos2;

            let mut int_safe_y1 = (*y1) as i32;
            let mut int_safe_x1 = (*x1) as i32;
            let mut int_safe_y2 = (*y2) as i32;
            let mut int_safe_x2 = (*x2) as i32;

            let (p1_diff_y, p1_diff_x) = (int_safe_y1 - int_safe_y2, int_safe_x1 - int_safe_x2);
            let (p2_diff_y, p2_diff_x) = (int_safe_y2 - int_safe_y1, int_safe_x2 - int_safe_x1);

            while horizontal_range.contains(&int_safe_x1) && vertical_range.contains(&int_safe_y1) {
                grid[int_safe_y1 as usize][int_safe_x1 as usize] = '#';
                int_safe_y1 += p1_diff_y;
                int_safe_x1 += p1_diff_x;

            }
            while horizontal_range.contains(&int_safe_x2) && vertical_range.contains(&int_safe_y2) {
                grid[int_safe_y2 as usize][int_safe_x2 as usize] = '#';
                int_safe_y2 += p2_diff_y;
                int_safe_x2 += p2_diff_x;
            }
        });
    });
    Some(count_in_grid(&grid, &'#'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
