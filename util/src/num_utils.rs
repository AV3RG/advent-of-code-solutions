use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

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

pub fn last_n_digits<T>(num: T, n: T) -> T
where T: Copy + PartialOrd + From<u16> + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Rem<Output = T> + Sub<Output = T> + DivAssign + AddAssign + MulAssign,
{
    let ten = T::from(10);
    let mut k = T::from(0);
    let mut modder = T::from(10);
    let one = T::from(1);
    while k < (n - one) {
        modder *= ten;
        k += one;
    }
    num % modder
}

pub fn first_n_digits<T>(mut num: T, n: T) -> T
where T: Copy + PartialOrd + From<u16> + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Rem<Output = T> + DivAssign + SubAssign,
{
    let total_digits = num_of_digits(num.clone());
    let mut k = total_digits - n;
    let ten = T::from(10);
    let zero = T::from(0);
    let one = T::from(1);
    while k > zero {
        num /= ten;
        k -= one;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(num_of_digits(100u64), 3);
        assert_eq!(last_n_digits(240, 2), 40);
        assert_eq!(last_n_digits(240, 1), 0);
    }
}
