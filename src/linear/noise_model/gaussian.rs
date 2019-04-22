use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::Dim;
use nalgebra::base::{DMatrix, DVector, MatrixN, VectorN};
use nalgebra::RealField;
use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub struct Gaussian<D: Dim, T: RealField = f64>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    dim: usize,
    sqrt_info: Option<MatrixN<T, D>>,
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField> GaussianNoise<D, T> for Gaussian<D, T>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    fn from_sqrtinfo(R: &MatrixN<T, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        let (m, n) = (R.nrows(), R.ncols());
        assert_eq!(m, n, "Non-square Matrix");
        if smart {
            if let Some(diagonal) = check_diagonal_upper(&R) {
                unimplemented!();
            }
        }

        Gaussian {
            dim: R.nrows(),
            sqrt_info: Some(R.to_owned()),
        }
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
        let (m, n) = (cov.nrows(), cov.ncols());
        assert_eq!(m, n, "Non-square Matrix");
        if smart {
            if let Some(diagonal) = check_diagonal_upper(cov) {
                unimplemented!();
            }
        }

        // NOTE: if cov = L'*L, then the square root information R can be found by
        // QR, as L.inverse() = Q*R, with Q some rotation matrix. However, R has
        // annoying sign flips with respect the simpler Information(inv(cov)),
        // hence we choose the simpler path here:
        let inv = cov.clone().try_inverse();
        Gaussian::from_information(&inv.unwrap(), false)
    }

    fn sqrt_info(&self) -> Option<&MatrixN<T, D>>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        if let Some(s) = &self.sqrt_info {
            return Some(s);
        }
        None
    }

    /**
     * Mahalanobis distance v'*R'*R*v = <R*v,R*v>
     */
    fn mahalanobis_dist(&self, v: &DVector<T>) -> T {
        unimplemented!()
    }
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField> NoiseModel<D, T> for Gaussian<D, T>
where
    DefaultAllocator: Allocator<T, D, D>,
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
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_mat(&self, m: &MatrixN<T, D>) -> MatrixN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        unimplemented!()
    }

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn distance(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_system<_D: Dim>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, _D>,
    {
        unimplemented!()
    }
}
