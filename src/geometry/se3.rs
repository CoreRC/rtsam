pub use crate::core::group::LieGroup;
pub use crate::core::manifold::Manifold;
use nalgebra::{Isometry, Matrix6, MatrixN, Real, Rotation3, Vector6, U6};

use nalgebra as na;

pub use na::Isometry3 as SE3;

impl<N: Real> LieGroup<N> for SE3<N> {
    type D = U6;

    fn between(&self, g: &Self) -> Self {
        return self.inverse() * g;
    }

    fn adjoint_map(&self) -> MatrixN<N, U6> {
        use crate::core::matrix::skew_symmetric;
        use na::U3;

        let mut res = MatrixN::<N, U6>::zeros();

        let R = self.rotation.to_rotation_matrix();;

        res.fixed_slice_mut::<U3, U3>(0, 0).copy_from(&R.matrix());
        res.fixed_slice_mut::<U3, U3>(0, 3).copy_from(
            &(skew_symmetric(self.translation.x, self.translation.y, self.translation.z) * R),
        );
        res.fixed_slice_mut::<U3, U3>(3, 3).copy_from(&R.matrix());

        res
    }

    fn logmap(R: &Self, optionalH: Option<&mut Matrix6<N>>) -> Vector6<N> {
        unimplemented!("NOT IMPLEMENTED");
    }

    fn expmap(omega: &Vector6<N>) -> Self {
        unimplemented!("NOT IMPLEMENTED");
    }

    #[inline]
    fn expmap_with_derivative(
        omega: &Vector6<N>,
        optionalH: Option<&mut Matrix6<N>>,
        _nearZero: bool,
    ) -> Self {
        unimplemented!("NOT IMPLEMENTED");
    }
}

impl<N: Real> Manifold for SE3<N> {
    type TangentVector = Vector6<N>;

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
        use na::Vector3;
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
}
