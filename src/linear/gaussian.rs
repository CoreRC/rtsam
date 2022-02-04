use crate::inference::factor::{Factor, KeyType};
use crate::linear::gaussian_like::GaussianLikeFactor;
use nalgebra as na;

pub struct GaussianFactor {}

impl Factor for GaussianFactor {
    fn num_keys(&self) -> usize {
        todo!()
    }

    fn key_at(&self, index: usize) -> KeyType {
        todo!()
    }
}

impl GaussianLikeFactor for GaussianFactor {
    fn augmented_jacobian(&self) -> na::DMatrix<f64> {
        unimplemented!()
    }

    fn jacobian(
        &self,
    ) -> (
        na::OMatrix<f64, na::Dynamic, na::Dynamic>,
        na::OVector<f64, na::Dynamic>,
    ) {
        unimplemented!()
    }

    fn augmented_information(&self) -> na::OMatrix<f64, na::Dynamic, na::Dynamic> {
        unimplemented!()
    }

    fn information(&self) -> na::OMatrix<f64, na::Dynamic, na::Dynamic> {
        unimplemented!()
    }

    fn hessian_diagonal(&self) -> Vec<(u64, na::OVector<f64, na::Dynamic>)> {
        unimplemented!()
    }

    fn hessian_block_diagonal(&self) -> Vec<(u64, na::OMatrix<f64, na::Dynamic, na::Dynamic>)> {
        unimplemented!()
    }
}
