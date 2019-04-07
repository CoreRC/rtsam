use std::fmt::Debug;
use nalgebra::base::{DVector, VectorN, MatrixMN, DMatrix};
use nalgebra::Real;
use nalgebra::base::dimension::{DimName};
use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;

#[allow(non_snake_case)]
pub trait NoiseModel<
    D: DimName,
    D1: DimName,
    D2: DimName,
    T: Real = f64
>: Debug + Eq {
    fn is_constrained(&self) -> bool;

    fn is_unit(&self) -> bool;

    fn dim(&self) -> usize;

    fn sigmas(&self) -> DVector<T>;

    fn whiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where DefaultAllocator: Allocator<T, D>;

    fn whiten_mat(&self, m: &MatrixMN<T, D1, D2>) -> MatrixMN<T, D1, D2>
    where DefaultAllocator: Allocator<T, D1, D2>;

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
        where DefaultAllocator: Allocator<T, D>;

    fn distance(&self, v: &VectorN<T, D>) -> T
        where DefaultAllocator: Allocator<T, D>;

    fn whiten_system<_D: DimName>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
        where DefaultAllocator: Allocator<T, _D>;
}

pub trait GaussianNoise<
    D: DimName,
    D1: DimName,
    D2: DimName,
    T: Real = f64
>: NoiseModel<D, D1, D2, T> {
    fn from_sqrtinfo() -> Self;

    fn from_information() -> Self;

    fn from_covariance() -> Self;

    fn mahalanobis_dist(&self, v: &[T]) -> T;
}