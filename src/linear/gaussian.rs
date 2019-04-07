use crate::inference::Conditional;
use crate::inference::Factor;

use nalgebra as na;

pub trait GaussianFactor: Factor {
    fn augmented_jacobian(&self) -> na::MatrixMN<f64, na::Dynamic, na::Dynamic>;

    fn jacobian(
        &self,
    ) -> (
        na::MatrixMN<f64, na::Dynamic, na::Dynamic>,
        na::VectorN<f64, na::Dynamic>,
    );

    fn augmented_information(&self) -> na::MatrixMN<f64, na::Dynamic, na::Dynamic>;

    fn information(&self) -> na::MatrixMN<f64, na::Dynamic, na::Dynamic>;

    fn hessian_diagonal(&self) -> Vec<(u64, na::VectorN<f64, na::Dynamic>)>;

    fn hessian_block_diagonal(&self) -> Vec<(u64, na::MatrixN<f64, na::Dynamic>)>;
}

pub trait GaussianConditional<F: GaussianFactor>: Conditional<F> {}
