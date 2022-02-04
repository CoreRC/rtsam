use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::Dim;
use nalgebra::base::{DMatrix, DVector, MatrixN, VectorN};
use nalgebra::RealField;

use super::*;

#[derive(Debug)]
pub struct Isotropic<D: Dim, T: RealField + Copy = f64> {
    dim: usize,
    sigma_: T,
    invsigma_: T,
    _phantom: std::marker::PhantomData<D>,
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField + Copy> GaussianNoise<D, T> for Isotropic<D, T>
where
    DefaultAllocator: Allocator<T, D>,
{
    fn from_sqrtinfo(R: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!() // TODO
    }

    fn from_information(info: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn from_covariance(cov: &MatrixN<T, D>, smart: bool) -> Self
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
        v.dot(v) * self.invsigma_ * self.invsigma_
    }
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField + Copy> NoiseModel<D, T> for Isotropic<D, T>
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

    fn whiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>,
    {
        v * self.invsigma_
    }

    fn whiten_mat(&self, m: &MatrixN<T, D>) -> MatrixN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>,
    {
        unimplemented!()
    }

    fn distance(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_system<_D: Dim>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, _D>,
    {
        unimplemented!()
    }
}
