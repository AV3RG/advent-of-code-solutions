use std::ops::{Add, Range};

pub fn convert_to_ranges<T: From<u16> + Copy + Add<Output = T> + PartialEq>(discrete: &Vec<T>) -> Vec<Range<T>> {
    let mut ranges = Vec::new();
    if discrete.is_empty() {
        return ranges;
    }
    let mut start = discrete[0];
    let mut end = discrete[0];
    for i in 1..discrete.len() {
        if discrete[i] == end + T::from(1) {
            end = discrete[i];
        } else {
            ranges.push(start..end);
            start = discrete[i];
            end = discrete[i];
        }
    }
    ranges.push(start..end);
    ranges
}
