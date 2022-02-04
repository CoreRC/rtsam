use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::Dim;
use nalgebra::base::{DMatrix, DVector, MatrixN, VectorN};
use nalgebra::RealField;
use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub struct Unit<D: Dim, T: RealField + Copy = f64>
where
    DefaultAllocator: Allocator<T, D>,
{
    dim: usize,
    sigmas_: VectorN<T, D>,
    invsigmas_: VectorN<T, D>,
    precisions_: VectorN<T, D>,
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField + Copy> GaussianNoise<D, T> for Unit<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn from_sqrtinfo(_R: &MatrixN<T, D>, _smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!() // TODO
    }

    fn from_information(_info: &MatrixN<T, D>, _smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn from_covariance(_cov: &MatrixN<T, D>, _smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!() // TODO
    }

    fn sqrt_info(&self) -> Option<&MatrixN<T, D>>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!() // TODO
    }

    /**
     * Mahalanobis distance v'*R'*R*v = <R*v,R*v>
     */
    fn mahalanobis_dist(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D>,
    {
        let w = self.whiten(v);
        w.dot(&w)
    }
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField + Copy> NoiseModel<D, T> for Unit<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn is_constrained(&self) -> bool {
        unimplemented!()
    }

    fn is_unit(&self) -> bool {
        unimplemented!()
    }

    fn dim(&self) -> usize {
        self.dim
    }

    fn sigmas(&self) -> DVector<T> {
        unimplemented!()
    }

    fn whiten(&self, _v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_mat(&self, _m: &MatrixN<T, D>) -> MatrixN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn unwhiten(&self, _v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>,
    {
        unimplemented!()
    }

    fn distance(&self, _v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_system<_D: Dim>(&self, _A: &[DMatrix<T>], _b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, _D>,
    {
        unimplemented!()
    }
}
