use crate::{geometry::Geometry, topology::Vertex};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Vertex {
    fn validate(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
        _: &Geometry,
    ) {
    }
}
