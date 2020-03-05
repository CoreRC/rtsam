use alga::general::{ClosedAdd, ClosedMul, ClosedNeg, ClosedSub, RealField, Ring, ComplexField, Field};
use nalgebra::Scalar;
use crate::symbolic::types::symbol::Size;

pub struct Matrix<T: Scalar> {
    name: String,
    m: Size,
    n: Size,
    _type: T,
    location: String
}