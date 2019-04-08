pub use crate::core::group::LieGroup;
pub use crate::core::manifold::Manifold;
use nalgebra::{Matrix6, MatrixN, Vector6, U6};

use nalgebra as na;

pub use na::Isometry3 as SE3;

#[allow(non_snake_case)]
impl LieGroup<f64> for SE3<f64> {
    type D = U6;

    fn between(&self, g: &Self) -> Self {
        self.inverse() * g
    }

    fn adjoint_map(&self) -> MatrixN<f64, U6> {
        use crate::core::matrix::skew_symmetric;
        use na::U3;

        let mut res = MatrixN::<f64, U6>::zeros();

        let R = self.rotation.to_rotation_matrix();

        res.fixed_slice_mut::<U3, U3>(0, 0).copy_from(&R.matrix());
        res.fixed_slice_mut::<U3, U3>(0, 3).copy_from(
            &(skew_symmetric(self.translation.x, self.translation.y, self.translation.z) * R),
        );
        res.fixed_slice_mut::<U3, U3>(3, 3).copy_from(&R.matrix());

        res
    }

    fn logmap(P: &Self, optionalH: Option<&mut Matrix6<f64>>) -> Vector6<f64> {
        use crate::core::matrix::*;
        use crate::geometry::so3::*;
        use na::{U1, U3};

        if let Some(_H) = optionalH {
            unimplemented!("NOT IMPLEMENTED");
        }

        let w = SO3::logmap(&P.rotation.to_rotation_matrix(), None);
        let T = P.translation.vector;
        let t = w.norm();
        if t < 1e-10 {
            let mut log = Vector6::zeros();
            log.fixed_slice_mut::<U3, U1>(0, 0).copy_from(&w);
            log.fixed_slice_mut::<U3, U1>(3, 0).copy_from(&T);
            log
        } else {
            let W = skew_symmetric_v(&(w / t));
            // Formula from Agrawal06iros, equation (14)
            // simplified with Mathematica, and multiplying in T to avoid matrix math
            let Tan = (0.5 * t).tan();
            let WT = W * T;
            let u = T - (0.5 * t) * WT + (1. - t / (2. * Tan)) * (W * WT);
            let mut log = Vector6::zeros();
            log.fixed_slice_mut::<U3, U1>(0, 0).copy_from(&w);
            log.fixed_slice_mut::<U3, U1>(3, 0).copy_from(&u);
            log
        }
    }

    fn expmap(xi: &Vector6<f64>) -> Self {
        Self::expmap_with_derivative(xi, None)
    }

    /** Modified from Murray94book version (which assumes w and v normalized?) */
    #[inline]
    fn expmap_with_derivative(xi: &Vector6<f64>, optionalH: Option<&mut Matrix6<f64>>) -> Self {
        use crate::geometry::so3::*;
        use na::Vector3;

        if let Some(_H) = optionalH {
            unimplemented!("NOT IMPLEMENTED");
        }

        // get angular velocity omega and translational velocity v from twist xi
        let (omega, v) = (
            Vector3::new(xi[0], xi[1], xi[2]),
            Vector3::new(xi[3], xi[4], xi[5]),
        );

        let R = SO3::expmap(&omega);
        let theta2 = omega.dot(&omega);
        if theta2 > std::f64::EPSILON {
            let t_parallel = omega * omega.dot(&v); // translation parallel to axis
            let omega_cross_v = omega.cross(&v); // points towards axis
            let t = (omega_cross_v - R * omega_cross_v + t_parallel) / theta2;
            SE3::from_parts(t.into(), R.into())
        } else {
            SE3::from_parts(v.into(), R.into())
        }
    }
}

impl Manifold for SE3<f64> {
    type TangentVector = Vector6<f64>;

    fn local(origin: &Self, other: &Self) -> Self::TangentVector {
        SE3::logmap(&origin.between(&other), None)
    }

    fn retract(origin: &Self, v: &Self::TangentVector) -> Self {
        origin * SE3::expmap(&v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_between() {
        use na::{Rotation3, Vector3};
        let a = SE3::new(Vector3::new(0.1, 0.2, 0.3), Vector3::new(0.1, 0.2, 0.3));
        let b = SE3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.1, 0.2, 0.3));

        assert_relative_eq!(
            a.between(&b).rotation.to_rotation_matrix(),
            Rotation3::identity(),
            epsilon = 0.01
        );
    }

    #[test]
    fn test_adjoint_map() {
        use na::Vector3;
        let a = SE3::new(Vector3::new(0.1, 0.2, 0.3), Vector3::new(0.1, 0.2, 0.3));

        println!("{:}", a.adjoint_map());
    }

    #[test]
    fn expmap_logmap_invariant() {
        let w = Vector6::new(1., 1.2, 1.3, 1., 1.4, 1.3);

        let exp = SE3::expmap_with_derivative(&w, None);

        assert_relative_eq!(w, SE3::logmap(&exp, None), epsilon = 0.001);
    }
}
