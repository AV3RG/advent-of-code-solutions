pub mod grid_utils;
pub mod num_utils;
pub mod combinatorics;
pub mod range_utils;
pub mod tuple_maths;
pub mod hex_utils;
pub mod bit_utils;
pub mod quad_utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
