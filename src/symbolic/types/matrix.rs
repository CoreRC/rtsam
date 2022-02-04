use crate::symbolic::types::symbol::Size;
use alga::general::{
    ClosedAdd, ClosedMul, ClosedNeg, ClosedSub, ComplexField, Field, RealField, Ring,
};
use nalgebra::Scalar;

pub struct Matrix<T: Scalar> {
    name: String,
    m: Size,
    n: Size,
    _type: T,
    location: String,
}
