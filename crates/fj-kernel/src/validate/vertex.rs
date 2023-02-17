use crate::objects::{GlobalVertex, SurfaceVertex};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for SurfaceVertex {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}

impl Validate for GlobalVertex {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
