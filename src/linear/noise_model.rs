use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::{Dim, DimName, U1};
use nalgebra::base::storage::Storage;
use nalgebra::base::{DMatrix, DVector, MatrixMN, MatrixN, SquareMatrix, VectorN};
use nalgebra::RealField;
use std::fmt::Debug;

#[allow(non_snake_case)]
pub trait NoiseModel<D: Dim, D1: Dim, D2: Dim, T: RealField = f64>: Debug {
    fn is_constrained(&self) -> bool;

    fn is_unit(&self) -> bool;

    fn dim(&self) -> usize;

    fn sigmas(&self) -> DVector<T>;

    fn whiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>;

    fn whiten_mat(&self, m: &MatrixMN<T, D1, D2>) -> MatrixMN<T, D1, D2>
    where
        DefaultAllocator: Allocator<T, D1, D2>;

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D>;

    fn distance(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D>;

    fn whiten_system<_D: Dim>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, _D>;
}

#[allow(non_snake_case)]
pub trait GaussianNoise<D: Dim, D1: Dim, D2: Dim, T: RealField = f64>:
    NoiseModel<D, D1, D2, T>
{
    fn from_sqrtinfo(R: &MatrixMN<T, D1, D2>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D1, D2>;

    fn from_information(info: &MatrixMN<T, D1, D2>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D1, D2>;

    fn from_covariance(cov: &MatrixMN<T, D1, D2>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D1, D2>;

    fn mahalanobis_dist(&self, v: &DVector<T>) -> T;
}

fn check_diagonal_upper<D1: Dim, D2: Dim, T: nalgebra::RealField>(
    mat: &MatrixMN<T, D1, D2>,
) -> Option<DVector<T>>
where
    DefaultAllocator: Allocator<T, D1, D2>,
{
    let (m, n) = (mat.nrows(), mat.ncols());
    let mut full = false;
    for i in 0..m {
        if !full {
            for j in i + 1..n {
                if mat[(i, j)].abs() > T::default_epsilon() {
                    full = true;
                    break;
                }
            }
        }
    }

    if full {
        None
    } else {
        let mut diag = DVector::identity(m);
        for i in 0..m {
            diag[i] = mat[(i, i)]
        }
        return Some(diag);
    }
}

#[derive(Debug)]
struct Gaussian<D1: Dim, D2: Dim, T: RealField = f64>
where
    DefaultAllocator: Allocator<T, D1, D2>,
{
    dim: usize,
    sqrt_info: Option<MatrixMN<T, D1, D2>>,
}

impl<D: Dim, D1: Dim, D2: Dim, T: RealField> GaussianNoise<D, D1, D2, T> for Gaussian<D1, D2, T>
where
    DefaultAllocator: Allocator<T, D1, D2>,
{
    fn from_sqrtinfo(R: &MatrixMN<T, D1, D2>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D1, D2>,
    {
        let (m, n) = (R.nrows(), R.ncols());
        assert_ne!(m, n, "Non-square Matrix");
        if smart {
            if let Some(diagonal) = check_diagonal_upper(&R) {}
        }

        Gaussian {
            dim: R.nrows(),
            sqrt_info: Some(R.to_owned()),
        }
    }

    fn from_information(info: &MatrixMN<T, D1, D2>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D1, D2>,
    {
        unimplemented!()
    }

    fn from_covariance(cov: &MatrixMN<T, D1, D2>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D1, D2>,
    {
        unimplemented!()
    }

    fn mahalanobis_dist(&self, v: &DVector<T>) -> T {
        unimplemented!()
    }
}

impl<D: Dim, D1: Dim, D2: Dim, T: RealField> NoiseModel<D, D1, D2, T> for Gaussian<D1, D2, T>
where
    DefaultAllocator: Allocator<T, D1, D2>,
{
    fn is_constrained(&self) -> bool {
        unimplemented!()
    }

    fn is_unit(&self) -> bool {
        unimplemented!()
    }

    fn dim(&self) -> usize {
        unimplemented!()
    }

    fn sigmas(&self) -> DVector<T> {
        unimplemented!()
    }

    fn whiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D1, D2> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_mat(&self, m: &MatrixMN<T, D1, D2>) -> MatrixMN<T, D1, D2>
    where
        DefaultAllocator: Allocator<T, D1, D2>,
    {
        unimplemented!()
    }

    fn unwhiten(&self, v: &VectorN<T, D>) -> VectorN<T, D>
    where
        DefaultAllocator: Allocator<T, D1, D2> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn distance(&self, v: &VectorN<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D1, D2> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn whiten_system<_D: Dim>(&self, A: &[DMatrix<T>], b: &VectorN<T, _D>)
    where
        DefaultAllocator: Allocator<T, D1, D2> + Allocator<T, _D>,
    {
        unimplemented!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaussian_model_construction() {
        let si = DMatrix::<f64>::identity(4, 4);
        let g = Gaussian::from_sqrtinfo(&si, false);
    }
}
