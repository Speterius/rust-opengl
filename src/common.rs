use crate::{Unit, Vector3, NORM_EPS};

pub fn normalize(v: Vector3) -> Unit<Vector3> {
    Unit::try_new(v, NORM_EPS).unwrap()
}
