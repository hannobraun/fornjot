use fj_math::Transform;

use crate::{objects::Objects, path::GlobalPath, validate::ValidationError};

use super::TransformObject;

impl TransformObject for GlobalPath {
    fn transform(
        self,
        transform: &Transform,
        _: &Objects,
    ) -> Result<Self, ValidationError> {
        match self {
            Self::Circle(curve) => {
                Ok(Self::Circle(transform.transform_circle(&curve)))
            }
            Self::Line(curve) => {
                Ok(Self::Line(transform.transform_line(&curve)))
            }
        }
    }
}
