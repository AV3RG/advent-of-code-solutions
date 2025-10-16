use std::ops::{AddAssign, Div, Rem};

pub fn count_bits<T>(num: &T) -> T
where T: Div<Output = T> + From<u16> + Rem<Output = T> + Copy + PartialEq + Eq + AddAssign
{
    let mut count = T::from(0);
    let div = T::from(2);
    let mut num = num.clone();
    let zero = T::from(0);
    while num != zero {
        count += num.rem(div);
        num = num.div(div);
    }
    count
}
