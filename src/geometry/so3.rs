pub use crate::core::group::LieGroup;
pub use crate::core::manifold::Manifold;
use nalgebra::{Matrix3, MatrixN, Vector3, U3};

pub use nalgebra::Rotation3 as SO3;
use std::f64::consts::PI;

#[allow(non_snake_case)]
impl LieGroup<f64> for SO3<f64> {
    type D = U3;

    fn between(&self, g: &Self) -> Self {
        self.inverse() * g
    }

    fn adjoint_map(&self) -> MatrixN<f64, U3> {
        *self.matrix()
    }

    fn logmap(R: &Self, optionalH: Option<&mut Matrix3<f64>>) -> Vector3<f64> {
        let (R11, R12, R13) = (R[(0, 0)], R[(0, 1)], R[(0, 2)]);
        let (R21, R22, R23) = (R[(1, 0)], R[(1, 1)], R[(1, 2)]);
        let (R31, R32, R33) = (R[(2, 0)], R[(2, 1)], R[(2, 2)]);

        let tr = R.into_inner().trace();

        let omega: Vector3<f64>;

        if (tr + 1.0).abs() < 1e-10 {
            if (R33 + 1.0).abs() > 1e-10 {
                omega = (PI / (2.0 + 2.0 * R33).sqrt()) * Vector3::new(R13, R23, 1.0 + R33);
            } else if (R22 + 1.0).abs() > 1e-10 {
                omega = (PI / (2.0 + 2.0 * R22).sqrt()) * Vector3::new(R12, 1.0 + R22, R32);
            } else {
                // if(abs(R.r1_.x()+1.0) > 1e-10)  This is implicit
                omega = (PI / (2.0 + 2.0 * R11).sqrt()) * Vector3::new(1.0 + R11, R21, R31);
            }
        } else {
            let magnitude: f64;

            let tr_3 = tr - 3.0; // always negative
            if tr_3 < -1e-7 {
                let theta = ((tr - 1.0) / 2.0).acos();
                magnitude = theta / (2.0 * (theta).sin());
            } else {
                // when theta near 0, +-2pi, +-4pi, etc. (trace near 3.0)
                // use Taylor expansion: theta \approx 1/2-(t-3)/12 + O((t-3)^2)
                magnitude = 0.5 - tr_3 * tr_3 / 12.0;
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
        let nearZero = theta2 <= std::f64::EPSILON;
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

        assert_relative_eq!(z, SO3::local(&v, &w), epsilon = 0.01);
    }

    #[test]
    fn test_manifold_retract() {
        let z = Vector3::new(0., 0., 0.1);
        let v = SO3::new(Vector3::z() * 0.1);
        let w = SO3::new(Vector3::z() * 0.2);

        assert_relative_eq!(w, SO3::retract(&v, &z), epsilon = 0.01);
    }
}
