pub trait Manifold {
    type TangentVector;

    fn local(origin: &Self, other: &Self) -> Self::TangentVector;

    fn retract(origin: &Self, v: &Self::TangentVector) -> Self;
}
