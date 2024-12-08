use std::collections::HashMap;
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


pub fn get_horizontal_line<T: std::fmt::Debug>(grid: &Vec<Vec<T>>, y: usize) -> Vec<&T> {
    grid.get(y).unwrap().iter().collect()
}

pub fn get_vertical_line<T>(grid: &Vec<Vec<T>>, x: usize) -> Vec<&T> {
    grid.into_iter().map(|row| &row[x]).collect()
}
