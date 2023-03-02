use crate::objects::Vertex;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Vertex {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
