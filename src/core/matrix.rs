use nalgebra as na;

pub fn skew_symmetric<N: na::Real>(wx: N, wy: N, wz: N) -> na::MatrixN<N, na::U3> {
    na::Matrix3::new(N::zero(), -wz, wy, wz, N::zero(), -wx, -wy, wx, N::zero())
}

pub fn skew_symmetric_v<N: na::Real>(v: &na::Vector3<N>) -> na::MatrixN<N, na::U3> {
    na::Matrix3::new(
        N::zero(),
        -v.z,
        v.y,
        v.z,
        N::zero(),
        -v.x,
        -v.y,
        v.x,
        N::zero(),
    )
}
