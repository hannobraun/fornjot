use fj_math::Transform;

use crate::{objects::Stores, path::GlobalPath};

use super::TransformObject;

impl TransformObject for GlobalPath {
    fn transform(self, transform: &Transform, _: &Stores) -> Self {
        match self {
            Self::Circle(curve) => {
                Self::Circle(transform.transform_circle(&curve))
            }
            Self::Line(curve) => Self::Line(transform.transform_line(&curve)),
        }
    }
}
