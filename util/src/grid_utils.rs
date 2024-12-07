pub fn find_in_grid<T: PartialEq>(grid: &Vec<Vec<T>>, search_value: &T) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == *search_value {
                return Some((y, x));
            }
        }
    }
    None
}

pub fn get_horizontal_line<T: std::fmt::Debug>(grid: &Vec<Vec<T>>, y: usize) -> Vec<&T> {
    grid.get(y).unwrap().iter().collect()
}

pub fn get_vertical_line<T>(grid: &Vec<Vec<T>>, x: usize) -> Vec<&T> {
    grid.into_iter().map(|row| &row[x]).collect()
}
