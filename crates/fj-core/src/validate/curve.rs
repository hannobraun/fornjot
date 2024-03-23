use crate::{
    geometry::Geometry,
    topology::Curve,
    validation::{ValidationConfig, ValidationError},
};

use super::Validate;

impl Validate for Curve {
    fn validate(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
        _: &Geometry,
    ) {
    }
}
