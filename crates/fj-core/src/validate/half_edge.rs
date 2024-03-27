use crate::{
    geometry::Geometry,
    topology::HalfEdge,
    validation::{ValidationConfig, ValidationError},
};

use super::Validate;

impl Validate for HalfEdge {
    fn validate(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
        _: &Geometry,
    ) {
    }
}
