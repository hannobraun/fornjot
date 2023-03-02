use crate::objects::GlobalVertex;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for GlobalVertex {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
