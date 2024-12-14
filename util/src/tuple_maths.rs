use std::ops::{Add, Div, Mul};

pub fn tuple_add<T: Add<Output = T>>(tuple1: (T, T), tuple2: (T, T)) -> (T, T) {
    (tuple1.0 + tuple2.0, tuple1.1 + tuple2.1)
}

pub fn tuple_scalar_multiply<T: Copy + Mul<Output = T>>(tuple: (T, T), scalar: T) -> (T, T) {
    (tuple.0 * scalar, tuple.1 * scalar)
}

pub fn tuple_compare<T: PartialEq>(lhs: (T, T), rhs: (T, T)) -> bool {
    lhs.0 == rhs.0 && lhs.1 == rhs.1
}

pub fn tuple_divide_scalar<T: Copy + Div<Output = T>>(tuple: (T, T), scalar: T) -> (T, T) {
    (tuple.0 / scalar, tuple.1 / scalar)
}

pub fn tuple_divide<T: Copy + Div<Output = T>>(tuple: (T, T), divisor: (T, T)) -> (T, T) {
    (tuple.0 / divisor.0, tuple.1 / divisor.1)
}
