pub use crate::core::group::LieGroup;
pub use crate::core::manifold::Manifold;
use nalgebra::{Matrix3, OMatrix, Vector3, U3};

pub use nalgebra::Rotation3 as SO3;
use std::f64::consts::PI;

#[allow(non_snake_case)]
impl LieGroup<f64> for SO3<f64> {
    type D = U3;

    fn between(&self, g: &Self) -> Self {
        self.inverse() * g
    }

    fn adjoint_map(&self) -> OMatrix<f64, U3, U3> {
        *self.matrix()
    }

    fn logmap(R: &Self, optionalH: Option<&mut Matrix3<f64>>) -> Vector3<f64> {
        let (R11, R12, R13) = (R[(0, 0)], R[(0, 1)], R[(0, 2)]);
        let (R21, R22, R23) = (R[(1, 0)], R[(1, 1)], R[(1, 2)]);
        let (R31, R32, R33) = (R[(2, 0)], R[(2, 1)], R[(2, 2)]);

        let tr = R.into_inner().trace();

        let omega: Vector3<f64>;

        // when trace == -1, i.e., when theta = +-pi, +-3pi, +-5pi, etc.
        // we do something special
        if tr + 1.0 < 1e-3 {
            if (R33 > R22) && (R33 > R11) {
                // R33 is the largest diagonal, a=3, b=1, c=2
                let W = R21 - R12;
                let Q1 = 2.0 + 2.0 * R33;
                let Q2 = R31 + R13;
                let Q3 = R23 + R32;
                let r = Q1.sqrt();
                let one_over_r = 1.0 / r;
                let norm = (Q1 * Q1 + Q2 * Q2 + Q3 * Q3 + W * W).sqrt();
                let sgn_w = if W < 0.0 { -1.0 } else { 1.0 };
                let mag = PI - (2.0 * sgn_w * W) / norm;
                let scale = 0.5 * one_over_r * mag;
                omega = sgn_w * scale * Vector3::new(Q2, Q3, Q1);
            } else if R22 > R11 {
                // R22 is the largest diagonal, a=2, b=3, c=1
                let W = R13 - R31;
                let Q1 = 2.0 + 2.0 * R22;
                let Q2 = R23 + R32;
                let Q3 = R12 + R21;
                let r = Q1.sqrt();
                let one_over_r = 1.0 / r;
                let norm = (Q1 * Q1 + Q2 * Q2 + Q3 * Q3 + W * W).sqrt();
                let sgn_w = if W < 0.0 { -1.0 } else { 1.0 };
                let mag = PI - (2.0 * sgn_w * W) / norm;
                let scale = 0.5 * one_over_r * mag;
                omega = sgn_w * scale * Vector3::new(Q3, Q1, Q2);
            } else {
                // R11 is the largest diagonal, a=1, b=2, c=3
                let W = R32 - R23;
                let Q1 = 2.0 + 2.0 * R11;
                let Q2 = R12 + R21;
                let Q3 = R31 + R13;
                let r = Q1.sqrt();
                let one_over_r = 1.0 / r;
                let norm = (Q1 * Q1 + Q2 * Q2 + Q3 * Q3 + W * W).sqrt();
                let sgn_w = if W < 0.0 { -1.0 } else { 1.0 };
                let mag = PI - (2.0 * sgn_w * W) / norm;
                let scale = 0.5 * one_over_r * mag;
                omega = sgn_w * scale * Vector3::new(Q1, Q2, Q3);
            }
        } else {
            let magnitude: f64;
            let tr_3 = tr - 3.0; // could be non-negative if the matrix is off orthogonal
            if tr_3 < -1e-6 {
                // this is the normal case -1 < trace < 3
                let theta = ((tr - 1.0) / 2.0).acos();
                magnitude = theta / (2.0 * theta.sin());
            } else {
                // when theta near 0, +-2pi, +-4pi, etc. (trace near 3.0)
                // use Taylor expansion: theta \approx 1/2-(t-3)/12 + O((t-3)^2)
                // see https://github.com/borglab/gtsam/issues/746 for details
                magnitude = 0.5 - tr_3 / 12.0 + tr_3 * tr_3 / 60.0;
            }
            omega = magnitude * Vector3::new(R32 - R23, R13 - R31, R21 - R12);
        }

        if let Some(_H) = optionalH {
            unimplemented!("optionalH NOT IMPLEMENTED");
            // *H = LogmapDerivative(omega);
        }

        omega
    }

    fn expmap(omega: &Vector3<f64>) -> Self {
        Self::expmap_with_derivative(omega, None)
    }

    #[inline]
    fn expmap_with_derivative(omega: &Vector3<f64>, optionalH: Option<&mut Matrix3<f64>>) -> Self {
        let theta2 = omega.dot(omega);
        let nearZero = theta2 <= f64::EPSILON;
        let (wx, wy, wz) = (omega.x, omega.y, omega.z);
        let W = Matrix3::new(0.0, -wz, wy, wz, 0.0, -wx, -wy, wx, 0.0);

        if !nearZero {
            let theta = theta2.sqrt();
            let sin_theta = theta.sin();
            let s2 = (theta / 2.).sin();
            let one_minus_cos = 2.0 * s2 * s2;
            let K = W / theta;
            let KK = K * K;

            if let Some(H) = optionalH {
                let a = one_minus_cos / theta;
                let b = 1.0 - sin_theta / theta;
                let dexp_ = Matrix3::identity() - a * K + b * KK;

                *H = dexp_;
            }

            SO3::from_matrix(&(Matrix3::identity() + sin_theta * K + one_minus_cos * KK))
        } else {
            if let Some(H) = optionalH {
                *H = Matrix3::identity() - 0.5 * W;
            }

            SO3::from_matrix(&(Matrix3::identity() + W))
        }
    }
}

impl Manifold for SO3<f64> {
    type TangentVector = Vector3<f64>;

    fn local(origin: &Self, other: &Self) -> Self::TangentVector {
        SO3::logmap(&origin.between(&other), None)
    }

    fn retract(origin: &Self, v: &Self::TangentVector) -> Self {
        origin * SO3::expmap(&v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_manifold_local() {
        let z = Vector3::new(0., 0., 0.2);
        let v = SO3::new(Vector3::z() * 0.1);
        let w = SO3::new(Vector3::z() * 0.3);

        assert_relative_eq!((z - SO3::local(&v, &w)).norm(), 0.0);
    }

    #[test]
    fn test_manifold_retract() {
        let z = Vector3::new(0., 0., 0.1);
        let v = SO3::new(Vector3::z() * 0.1);
        let w = SO3::new(Vector3::z() * 0.2);

        assert_relative_eq!(
            (w.matrix() - SO3::retract(&v, &z).matrix()).norm(),
            0.0,
            epsilon = 1e-10
        );
    }
}
