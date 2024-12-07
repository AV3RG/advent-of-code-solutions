use std::ops::{Add, Div};

pub fn num_of_digits<T>(mut n: T) -> T
where
    T: Copy + PartialOrd + From<u16> + Div<Output = T> + Add<Output = T>,
{
    let zero = T::from(0);
    let ten = T::from(10);
    let one = T::from(1);
    let mut count = zero;

    while n > zero {
        count = count + one;
        n = n / ten;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = num_of_digits(100u64);
        assert_eq!(result, 3);
    }
}
