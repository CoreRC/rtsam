use crate::inference::Conditional;
use crate::inference::Factor;

use nalgebra as na;

pub trait GaussianLikeFactor: Factor {
    fn augmented_jacobian(&self) -> na::OMatrix<f64, na::Dynamic, na::Dynamic>;

    fn jacobian(
        &self,
    ) -> (
        na::OMatrix<f64, na::Dynamic, na::Dynamic>,
        na::OVector<f64, na::Dynamic>,
    );

    fn augmented_information(&self) -> na::OMatrix<f64, na::Dynamic, na::Dynamic>;

    fn information(&self) -> na::OMatrix<f64, na::Dynamic, na::Dynamic>;

    fn hessian_diagonal(&self) -> Vec<(u64, na::OVector<f64, na::Dynamic>)>;

    fn hessian_block_diagonal(&self) -> Vec<(u64, na::OMatrix<f64, na::Dynamic, na::Dynamic>)>;
}

pub trait GaussianConditional<F: GaussianLikeFactor>: Conditional<F> {}
