use nalgebra::base::allocator::Allocator;
use nalgebra::base::default_allocator::DefaultAllocator;
use nalgebra::base::dimension::Dim;
use nalgebra::base::{DMatrix, DVector, OMatrix, OVector};
use nalgebra::DimSub;
use nalgebra::RealField;
use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub struct Gaussian<D: Dim, T: RealField + Copy = f64>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    dim: usize,
    sqrt_info: Option<OMatrix<T, D, D>>,
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField + Copy> GaussianNoise<D, T> for Gaussian<D, T>
where
    DefaultAllocator: Allocator<T, D, D>,
{
    fn from_sqrtinfo(R: &OMatrix<T, D, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        let (m, n) = (R.nrows(), R.ncols());
        assert_eq!(m, n, "Non-square Matrix");
        if smart {
            if let Some(_diagonal) = check_diagonal_upper(&R) {
                unimplemented!();
            }
        }

        Gaussian {
            dim: R.nrows(),
            sqrt_info: Some(R.to_owned()),
        }
    }

    fn from_information(info: &OMatrix<T, D, D>, _smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
        D: DimSub<nalgebra::Dynamic>,
    {
        use nalgebra::Cholesky;

        let (m, n) = (info.nrows(), info.ncols());
        assert_eq!(m, n, "Non-square Matrix");

        let llt = Cholesky::new(info.clone()).unwrap();
        let R = llt.l_dirty();

        return Gaussian {
            dim: m,
            sqrt_info: Some(R.transpose()),
        };
    }

    fn from_covariance(cov: &OMatrix<T, D, D>, smart: bool) -> Self
    where
        DefaultAllocator: Allocator<T, D, D>,
        D: DimSub<nalgebra::Dynamic>,
    {
        let (m, n) = (cov.nrows(), cov.ncols());
        assert_eq!(m, n, "Non-square Matrix");
        if smart {
            if let Some(_diagonal) = check_diagonal_upper(cov) {
                unimplemented!();
            }
        }

        // NOTE: if cov = L'*L, then the square root information R can be found by
        // QR, as L.inverse() = Q*R, with Q some rotation matrix. However, R has
        // annoying sign flips with respect to the simpler Information(inv(cov)),
        // hence we choose the simpler path here:
        let inv = cov.clone().try_inverse();
        Gaussian::from_information(&inv.unwrap(), false)
    }

    fn sqrt_info(&self) -> Option<&OMatrix<T, D, D>>
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
    fn mahalanobis_dist(&self, v: &OVector<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D>,
    {
        let w = self.whiten(v);
        w.dot(&w)
    }
}

#[allow(non_snake_case)]
impl<D: Dim, T: RealField + Copy> NoiseModel<D, T> for Gaussian<D, T>
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

    fn whiten(&self, v: &OVector<T, D>) -> OVector<T, D>
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        if let Some(R) = self.sqrt_info() {
            R * v
        } else {
            panic!("SqrtInfo Undefined")
        }
    }

    fn whiten_mat(&self, m: &OMatrix<T, D, D>) -> OMatrix<T, D, D>
    where
        DefaultAllocator: Allocator<T, D, D>,
    {
        if let Some(R) = self.sqrt_info() {
            R * m
        } else {
            panic!("SqrtInfo Undefined")
        }
    }

    fn unwhiten(&self, _v: &OVector<T, D>) -> OVector<T, D>
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        unimplemented!()
    }

    fn distance(&self, v: &OVector<T, D>) -> T
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, D>,
    {
        self.mahalanobis_dist(v)
    }

    fn whiten_system<_D: Dim>(&self, _A: &[DMatrix<T>], _b: &OVector<T, _D>)
    where
        DefaultAllocator: Allocator<T, D, D> + Allocator<T, _D>,
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Matrix4;

    #[test]
    fn gaussian_model_construction() {
        let si = DMatrix::<f64>::identity(4, 4);
        let g = Gaussian::from_sqrtinfo(&si, false);

        let se = Matrix4::<f64>::identity();
        let ge = Gaussian::from_information(&se, false);

        let lhs = g.sqrt_info().unwrap();
        let rhs = ge.sqrt_info().unwrap();
        assert_eq!((lhs - rhs).norm(), 0.0);

        println!("{:#?}", ge.sqrt_info());
    }

    #[test]
    fn sqrt_info_vs_cov_invariant() {
        let _si = DMatrix::<f64>::identity(4, 4);
        let _cm = DMatrix::<f64>::identity(4, 4);
    }
}
