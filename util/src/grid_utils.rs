use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

pub fn find_first_in_grid<T: PartialEq>(grid: &Vec<Vec<T>>, search_value: &T) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == *search_value {
                return Some((y, x));
            }
        }
    }
    None
}

pub fn find_all_in_grid<T: PartialEq>(grid: &Vec<Vec<T>>, search_value: &T) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == *search_value {
                result.push((y, x));
            }
        }
    }
    result
}

pub fn count_in_grid<T: PartialEq>(grid: &Vec<Vec<T>>, search_value: &T) -> u32 {
    let mut count = 0;
    for row in grid.iter() {
        for value in row.iter() {
            if *value == *search_value {
                count += 1;
            }
        }
    }
    count
}

pub fn grid_item_positions<T: Eq + Hash>(grid: &Vec<Vec<T>>) -> HashMap<&T, Vec<(usize, usize)>> {
    let mut result = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            result.entry(value).or_insert_with(Vec::new).push((y, x));
        }
    }
    result
}

pub fn print_grid<T: Debug>(grid: &Vec<Vec<T>>) {
    for row in grid.iter() {
        println!("{:?}", row);
    }
}

pub fn get_manhattan_neighbours(pos: (usize, usize), m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let (y, x) = pos;
    if x > 0 {
        result.push((y, x - 1));
    }
    if x < m - 1 {
        result.push((y, x + 1));
    }
    if y > 0 {
        result.push((y - 1, x));
    }
    if y < n - 1 {
        result.push((y + 1, x));
    }
    result
}

pub fn get_vertical_neighbours(pos: (usize, usize), n: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let (y, x) = pos;
    if y > 0 {
        result.push((y - 1, x));
    }
    if y < n - 1 {
        result.push((y + 1, x));
    }
    result
}

pub fn get_horizontal_neighbours(pos: (usize, usize), m: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let (y, x) = pos;
    if x > 0 {
        result.push((y, x - 1));
    }
    if x < m - 1 {
        result.push((y, x + 1));
    }
    result
}

pub fn get_horizontal_line<T>(grid: &Vec<Vec<T>>, y: usize) -> Vec<&T> {
    grid.get(y).unwrap().iter().collect()
}

pub fn get_vertical_line<T>(grid: &Vec<Vec<T>>, x: usize) -> Vec<&T> {
    grid.into_iter().map(|row| &row[x]).collect()
}

pub fn get_similar_region_around<T>(grid: &Vec<Vec<T>>, pos: (usize, usize), m: usize, n: usize, matcher: fn(&T, &T) -> bool) -> HashSet<(usize, usize)> {
    let mut consumed = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if consumed.contains(&current) {
            continue;
        }
        consumed.insert(current);
        let neighbours = get_manhattan_neighbours(current, m, n);
        for neighbour in neighbours {
            if consumed.contains(&neighbour) {
                continue;
            }
            if matcher(&grid[current.0][current.1], &grid[neighbour.0][neighbour.1]) {
                queue.push_back(neighbour);
            }
        }
    }
    consumed
}

pub fn largest_island<T>(grid: &Vec<Vec<T>>, m: usize, n: usize, skipper: fn(&T) -> bool, matcher: fn(&T, &T) -> bool) -> usize {
    let mut consumed = HashSet::new();
    let mut max_size = 0;
    for y in 0..n {
        for x in 0..m {
            if skipper(&grid[y][x]) {
                continue;
            }
            if consumed.contains(&(y, x)) {
                continue;
            }
            let region = get_similar_region_around(grid, (y, x), m, n, matcher);
            let n = region.len();
            consumed.extend(region);
            max_size = max_size.max(n);
        }
    }
    max_size
}
