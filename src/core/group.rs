use crate::core::manifold::Manifold;
use alga::general::{AbstractSemigroup, Multiplicative};
use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, DimName, MatrixN, Scalar, VectorN};
use std::fmt::Debug;
use std::ops::Mul;

#[allow(non_snake_case)]
pub trait LieGroup<N>: Debug + Sized + Copy
where
    Self: Mul<Self, Output = Self>,
    Self: for<'a> Mul<&'a Self, Output = Self>,
    for<'a> &'a Self: Mul<Self, Output = Self>,
    for<'a, 'b> &'a Self: Mul<&'b Self, Output = Self>,
    N: Scalar + num::Zero + num::One,
    Self: Manifold,
{
    type D: DimName;

    fn compose(&self, g: &Self) -> Self {
        self * g
    }

    fn between(&self, g: &Self) -> Self;

    fn adjoint_map(&self) -> MatrixN<N, Self::D>
    where
        DefaultAllocator: Allocator<N, Self::D, Self::D>;

    // TODO(fan): H now does not work
    fn logmap(R: &Self, H: Option<&mut MatrixN<N, Self::D>>) -> VectorN<N, Self::D>
    where
        DefaultAllocator: Allocator<N, Self::D> + Allocator<N, Self::D, Self::D>;

    fn expmap(omega: &VectorN<N, Self::D>) -> Self
    where
        DefaultAllocator: Allocator<N, Self::D>;

    fn expmap_with_derivative(
        omega: &VectorN<N, Self::D>,
        H: Option<&mut MatrixN<N, Self::D>>,
    ) -> Self
    where
        DefaultAllocator: Allocator<N, Self::D> + Allocator<N, Self::D, Self::D>;
}

#[cfg(test)]
mod tests {
    use crate::geometry::so3::*;
    use finitediff::FiniteDiff;
    use nalgebra::{Matrix3, Vector3};
    use std::f64::consts::PI;

    #[test]
    fn compose_works() {
        let r1 = SO3::<f64>::from_axis_angle(&Vector3::x_axis(), PI);
        let r2 = SO3::<f64>::from_axis_angle(&Vector3::x_axis(), -PI);

        assert_relative_eq!((r1.compose(&r2).matrix() - SO3::identity().matrix()).norm(), 0.0);
    }

    #[test]
    fn between_works() {
        let r1 = SO3::<f64>::from_axis_angle(&Vector3::x_axis(), 1. * PI);
        let r2 = SO3::<f64>::from_axis_angle(&Vector3::x_axis(), -1. * PI);

        assert_relative_eq!((r1.between(&r2).matrix() - SO3::identity().matrix()).norm(), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn adjoint_map_works() {
        let r1 = SO3::<f64>::from_axis_angle(&Vector3::x_axis(), 1. * PI);
        let r2 = SO3::<f64>::from_axis_angle(&Vector3::x_axis(), 3. * PI);

        assert_relative_eq!((r1.adjoint_map() - r2.adjoint_map()).norm(), 0.0, epsilon = 1e-10);
    }

    // Left trivialized Derivative of exp(w) wrpt w:
    // How does exp(w) change when w changes?
    // We find a y such that: exp(w) exp(y) = exp(w + dw) for dw --> 0
    // => y = log (exp(-w) * exp(w+dw))
    fn dexp_numeric(w: Vector3<f64>, dw: Vector3<f64>) -> Vector3<f64> {
        SO3::logmap(&(SO3::expmap(&-w) * SO3::expmap(&(w + dw))), None)
    }

    #[test]
    fn expmap_works_1() {
        let w = Vector3::new(0.1, 0.27, -0.2);
        let mut actual_dexp = Matrix3::<f64>::identity();

        let f = |x: &Vec<f64>| -> Vec<f64> {
            let arr: [f64; 3] = dexp_numeric(w, Vector3::new(x[0], x[1], x[2])).into();
            arr.to_vec()
        };

        let w_: [f64; 3] = w.into();
        let expected_dexp_ = w_.to_vec().central_jacobian(&f);
        let expected_dexp: Matrix3<f64> =
            Matrix3::from_iterator(expected_dexp_.iter().flatten().cloned());

        SO3::expmap_with_derivative(&w, Some(&mut actual_dexp));

        assert_relative_eq!((actual_dexp - expected_dexp).norm(), 0.0, epsilon = 0.02);
    }

    #[test]
    fn expmap_works_2() {
        let w = Vector3::new(10., 20., 30.);
        let mut actual_dexp = Matrix3::<f64>::identity();

        let f = |x: &Vec<f64>| -> Vec<f64> {
            let arr: [f64; 3] = dexp_numeric(w, Vector3::new(x[0], x[1], x[2])).into();
            arr.to_vec()
        };

        let w_: [f64; 3] = w.into();
        let expected_dexp_ = w_.to_vec().central_jacobian(&f);
        let expected_dexp: Matrix3<f64> =
            Matrix3::from_iterator(expected_dexp_.iter().flatten().cloned());

        SO3::expmap_with_derivative(&w, Some(&mut actual_dexp));

        assert_relative_eq!((actual_dexp - expected_dexp).norm(), 0.0, epsilon = 0.01);
    }

    #[test]
    fn expmap_logmap_invariant() {
        let w = Vector3::new(1., 1.2, 1.3);
        let mut dexp = Matrix3::<f64>::identity();

        let exp = SO3::expmap_with_derivative(&w, Some(&mut dexp));

        assert_relative_eq!((w - SO3::logmap(&exp, None)).norm(), 0.0, epsilon = 1e-10);
    }
}
