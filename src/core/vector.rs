extern crate nalgebra as na;

use self::na::storage::Storage;
use self::na::{DimName, U1};
pub use na::{Vector, Vector1, Vector2, Vector3, Vector4, Vector5, Vector6, VectorN};

pub fn linear_dependent<D, S>(vec1: &Vector<f64, D, S>, vec2: &Vector<f64, D, S>, tol: f64) -> bool
where
    D: DimName,
    S: Storage<f64, D, U1>,
{
    let mut flag = false;
    let mut scale = 1.0;

    for i in 0..vec1.len() {
        if ((vec1[i].abs() > tol) && (vec2[i].abs() < tol))
            || ((vec1[i].abs() < tol) && (vec2[i].abs() > tol))
        {
            return false;
        }

        if vec1[i] == 0.0 && vec2[i] == 0.0 {
            continue;
        }

        if !flag {
            scale = vec1[i] / vec2[i];
            flag = true;
        } else if (vec1[i] - vec2[i] * scale).abs() > tol {
            return false;
        }
    }

    return flag;
}

#[cfg(test)]
mod tests {
    use super::{Vector3, Vector6};
    use crate::core::vector::linear_dependent;

    #[test]
    fn vector_component_works() {
        assert_eq!(Vector3::x(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(Vector3::y(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Vector3::z(), Vector3::new(0.0, 0.0, 1.0));

        assert_eq!(
            Vector6::w(),
            Vector6::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            "w()"
        );
        assert_eq!(
            Vector6::a(),
            Vector6::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
            "a()"
        );
        assert_eq!(
            Vector6::b(),
            Vector6::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            "b()"
        );
    }

    #[test]
    fn linear_dependent_works() {
        assert_eq!(
            linear_dependent(&Vector6::<f64>::x(), &Vector6::x(), 0.01),
            true
        );

        assert_eq!(
            linear_dependent(&Vector3::<f64>::x(), &Vector3::y(), 0.01),
            false
        );

        assert_eq!(
            linear_dependent(&(Vector3::<f64>::x() * 3.5), &Vector3::x(), 0.01),
            true
        );

        assert_eq!(
            linear_dependent(
                &(Vector3::<f64>::x() * 3.5 + 0.01 * Vector3::y()),
                &Vector3::x(),
                0.001
            ),
            false
        );

        assert_eq!(
            linear_dependent(
                &(Vector3::<f64>::x() * 3.5 + 0.001 * Vector3::y()),
                &Vector3::x(),
                0.001
            ),
            true
        );
    }
}
