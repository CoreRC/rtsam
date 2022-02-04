use crate::symbolic::types::symbol::Size;

use nalgebra::Scalar;

pub struct Matrix<T: Scalar> {
    name: String,
    m: Size,
    n: Size,
    _type: T,
    location: String,
}
