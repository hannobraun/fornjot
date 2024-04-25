use crate::{
    geometry::Geometry,
    topology::Cycle,
    validation::{ValidationConfig, ValidationError},
};

use super::Validate;

impl Validate for Cycle {
    fn validate(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
        _: &Geometry,
    ) {
    }
}
