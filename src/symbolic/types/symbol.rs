
use nalgebra::Scalar;

pub struct Symbol<T: Scalar> {
    name: String,
    value: Option<T>,
}

pub type Size = Symbol<u64>;
