pub fn find_in_grid<T: PartialEq>(grid: &Vec<Vec<T>>, value: &T) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == *value {
                return Some((x, y));
            }
        }
    }
    None
}
