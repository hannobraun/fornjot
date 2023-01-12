use crate::objects::Solid;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Solid {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}
