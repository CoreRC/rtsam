use crate::inference::Conditional;
use crate::inference::Factor;

use nalgebra as na;
use nalgebra::allocator::Allocator;

trait GaussianFactor: Factor {
    fn augmented_jacobian(&self) -> na::MatrixMN<f64, na::Dynamic, na::Dynamic>;

    fn jacobian(
        &self,
    ) -> (
        na::MatrixMN<f64, na::Dynamic, na::Dynamic>,
        na::VectorN<f64, na::Dynamic>,
    )
    where
        na::DefaultAllocator:
            Allocator<f64, na::Dynamic, na::Dynamic> + Allocator<f64, na::Dynamic, na::U1>;
}

trait GaussianConditional<F: GaussianFactor>: Conditional<F> {}
