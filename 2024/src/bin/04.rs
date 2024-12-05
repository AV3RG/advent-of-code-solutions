advent_of_code::solution!(4);

fn check_horizontal(y: usize, x: usize, grid: &Vec<Vec<char>>, m: usize) -> u32 {
    let mut count = 0;
    if x + 3 < m && grid[y][x + 1] == 'A' && grid[y][x + 2] == 'M' && grid[y][x + 3] == 'X' {
        count += 1;
    }
    if x >= 3 && grid[y][x - 1] == 'A' && grid[y][x - 2] == 'M' && grid[y][x - 3] == 'X' {
        count += 1;
    }
    count
}

fn check_vertical(y: usize, x: usize, grid: &Vec<Vec<char>>, n: usize) -> u32 {
    let mut count = 0;
    if y + 3 < n && grid[y + 1][x] == 'A' && grid[y + 2][x] == 'M' && grid[y + 3][x] == 'X' {
        count += 1;
    }
    if y >= 3 && grid[y - 1][x] == 'A' && grid[y - 2][x] == 'M' && grid[y - 3][x] == 'X' {
        count += 1;
    }
    count
}

fn check_diagonals(y: usize, x: usize, grid: &Vec<Vec<char>>, m: usize, n: usize) -> u32 {
    let mut count = 0;
    if y + 3 < n && x + 3 < n && grid[y + 1][x + 1] == 'A' && grid[y + 2][x + 2] == 'M' && grid[y + 3][x + 3] == 'X' {
        count += 1
    }
    if y + 3 < n && x >= 3 && grid[y + 1][x - 1] == 'A' && grid[y + 2][x - 2] == 'M' && grid[y + 3][x - 3] == 'X' {
        count += 1
    }
    if y >= 3 && x >= 3 && grid[y - 1][x - 1] == 'A' && grid[y - 2][x - 2] == 'M' && grid[y - 3][x - 3] == 'X' {
        count += 1
    }
    if y >= 3 && x + 3 < n  && grid[y - 1][x + 1] == 'A' && grid[y - 2][x + 2] == 'M' && grid[y - 3][x + 3] == 'X' {
        count += 1
    }
    count
}

fn overall_check(y: usize, x: usize, grid: &Vec<Vec<char>>, m: usize, n: usize) -> u32 {
    if grid[y][x] != 'S' {
        return 0;
    }
    let mut counter = 0;
    counter += check_horizontal(y, x, grid, m);
    counter += check_vertical(y, x, grid, n);
    counter += check_diagonals(y, x, grid, m, n);
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<Vec<_>>>();
    let (n, m) = (grid.len(), grid[0].len());
    let sol = grid.iter().enumerate().map(|(y, line)| {
        return line.iter().enumerate().map(|(x, c)| {
            return overall_check(y, x, &grid, m, n)
        }).sum::<u32>()
    }).sum::<u32>();
    Some(sol)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.trim().lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<Vec<_>>>();
    let (n, m) = (grid.len(), grid[0].len());
    let sol = grid.iter().enumerate().map(|(y, line)| {
        return line.iter().enumerate().map(|(x, c)| {
            if *c != 'A' {
                return 0
            }
            if y == 0 || x == 0 || y == n - 1 || x == m - 1 {
                return 0
            }
            let chars = [grid[y+1][x-1], grid[y+1][x+1], grid[y-1][x+1], grid[y-1][x-1]];
            if !chars.iter().all(|c| *c == 'M' || *c == 'S') {
                return 0
            }
            if chars[0] == chars[2] || chars[1] == chars[3] {
                return 0
            }
            return 1
        }).sum::<u32>();
    }).sum::<u32>();
    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
